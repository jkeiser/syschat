Syschat is an ultra-simple anonymous message board.

Quickstart
----------

Run this (prerequisites: [Rust](https://www.rust-lang.org/tools/install) and [Node](https://nodejs.org/en/download/)):

  ```sh
  cargo run
  ```

And open your browser to [http://localhost:3000](http://localhost:3000).

Overview
--------

One or more users can open up Svelte web clients, send messages, and see all messages anyone else
sends (or has ever sent). Messages are managed in memory in a Rust+Axum API server. Clients poll
periodically for new messages.

An apparent Svelte live reloading bug cost me a good bit of time as well, forcing one or two
tradeoffs that I might not have had to make otherwise. I am still happy with the resulting codebase,
though. When time gets short you make a *smaller* thing you are proud of, not a bigger thing that
doesn't work.

The most significant tradeoffs here are:

1. The decision for clients to poll periodically for new messages rather than receive live updates
   immediately through a websocket. This decision impacts the perceived speed of the application, and
   increases the network and compute load. If there was one thing that should be done differently with
   more time, this would be it.

2. No end to end tests. Tests are at the API level. E2E tests are really important for actually
   knowing if your application works, and is probably the other big thing I'd want to do before
   feeling like this was truly "done" for what it is. However, E2E tests are notoriously finicky to
   get stable and not recommended for a 4-hour tour.

3. There is also a potential injection attack. This very possibly isn't real, but we can't ship
   without knowing for sure and hardening against it.

More detailed information on all these tradeoffs comes in the next few sections.

Project Structure
-----------------

The most important source files in the project: 

* `build.rs`: this runs `npm run build` when you `cargo build`
* `src/main.rs`: web and API server
  - `app()` defines routes and `fallback_handler` serves web client files
  - `list_messages` and `send_message` implement API endpoints
  - `Arc<RwLock<MessageBoard` stores state, wrapped in an `Arc<RwLock>>` for safe concurrent access to the `Vec<Message>` list
* `src/tests.rs`: API tests run during `cargo test`
* `client/routes/+page.svelte`: web client served as `/`
* `client/routes/NewMessage.svelte`: "send message" UI and implementation
* `client/routes/Live messages.svelte`: shows messages and keeps them up to date

Tech Decisions
--------------

These don't affect features directly, but do affect the development and operational experience.

* **Svelte for frontend.**

  I don't have a super strong preference on frontend frameworks, though I've enjoyed Vue and Svelte
  more than React. Vue would be a better choice in some ways because System Initiative uses it, and
  it's important to pick techs your team will be comfortable with. But I really like the compiled
  approach Svelte takes and wanted to explore it a little more :)

  This decision to use Svelte did cost me a couple of hours on what seems like a bug: reactivity
  seemed to stop working almost immediately after loading, and async fetches to get more messages
  would return but not update the UI. Eventually I narrowed it down to vite+svelte's live reloading
  development server, and abandoned that in favor of using static compiled JS files, which let me
  move forward.

* **Rust for backend.**

  Rust is happiness, especially for things you need to be rock-solid.

* **Axum server.**

  Never used this before--somehow I've never done a webserver in Rust, just compilers and non-HTTP
  servers. Axum seems fine though.

* **No CSS framework.**

  For such a simple application with a single page, this caused more indirection than necessary,
  and I didn't want to futz with another thing. But CSS frameworks can be super useful when you have
  a lot of pages and/or a lot of people maintaining the style.

* **The Rust server serves the statically compiled Svelte as well as the API.**

  The root of the Rust server serves the Javascript and HTML files compiled from Svelte. This allows
  for simpler service packaging and management, and allows us to avoid CORS issues talking from
  frontend to backend since they are the same origin.
  
  It would be slightly better to compile the static files into the executable rather than require
  them to be in a specific place on the drive. This would vastly simplify the deployment of the
  server (just ship the executable), and remove its dependency on the filesystem as well,
  simplifying management further.

* **The server stores all message history as an array in memory.**

  This is in the spec, but the real reason is limited dev time: getting persistence right (and
  reliable) takes time.

  The lack of a limit on message history means the process will eventually run out of memory
  and crash. This is mitigated somewhat since the server will successfully start fresh (with no
  messages) next time. We already accept that the server will lose all messages on restart due to
  the in-memory design, and this is just one more way that can happen.

  Ultimately if this were shipping, persistence would be a requirement. Doing it right would allow a
  much larger message limit as a side effect (though theoretically, everything runs out, it just
  depends how long you wait).
  
  The problem could be mitigated without persistence by implementing a limit and destroying the
  oldest messages when it would grow above that limit.

* **Many concurrent reads, one write at a time, mediated by `Arc<RwLock<MessageBoard>>`.**

  `RwLock` prevents reading existing messages while adding a new one, and prevents concurrent writes.
  
  It's hard to see this truly impacting the application given the low (for a computer) rate of
  messages sent. Theoretically at really heavy scale there could be high lock contention, causing
  high latency and other nasty things like CPU spinning. However, the only time the application will
  lock for more than a few instructions is when the messages vector is grown, which happens
  infrequently unless messages are sent at a rate too fast for any user to follow.

  If it's cheap though, it's worth doing. There are Rust packages that provide append-only lock-free
  lists that allow reads (and sometimes writes) to proceed without any locking at all. To the extent
  one of these is well-tested and can be dropped in cheaply, it will save at least a few
  instructions on every read, which *does* happen frequently.

Features / Design
-----------------

These are some of the major features and design decisions, tradeoffs, and things that I might have
done if I had more time.

* **The client loads all messages on startup.**

  This allows the user to see what's been happening in the past and is important user experience.

  The alternative design would be to show only new messages, but that means knowing nothing about
  what anyone else has ever said, possibly repeating stuff, and having nothing to respond to.

* **The client updates live by pulling new messages from the server once per second.**

  Live updates for new messages is more engaging and less annoying to the user than hitting reload
  over and over.

  The polling method, however, means updates are delayed. It also costs a lot more network than a
  web socket, as you have to do an entire back-and-forth TCP connection and HTTP GET each second,
  headers included, even if there are no new messages. A web socket only needs a periodic heartbeat
  to keep the connection alive and you can just send new data as it arrives.

  **If I had more time,** I would serve live message updates over a web socket and only listen for
  new messages after the initial load.

* **Newest messages are at the top.**

  This allows new messages to be close to where the eye is.
  
  The drawback is that generally computers prefer if you add things to the end of lists, and
  frontend frameworks are no exception. I'm a little worried that Svelte might completely obliterate
  and recreate the entire DOM for all existing messages each time a single new message comes in. I
  haven't really researched how Svelte handles changes to lists, but I can imagine frameworks
  comparing old and new lists element by element, and when the second list is just shifted by one,
  all elements will compare different! (Svelte probably worried about this too, so maybe it's not
  an issue.)

  **If I had more time,** I'd have investigated Svelte at some point to see whether this is actually
  causing massive DOM upheaval or not. At the very least, it's something I need to know to be able
  to write good Svelte code!

* **Sending a message accepts arbitrary text.**

  This is a huge warning sign for injection attacks. There aren't obvious ones on my very lazy
  first thought, but I wouldn't ship this without further thinking.

  **If I had more time,** I would investigate and fix injection attacks in:

  - Svelte's `&lt;div class="method"&gt;{message}&lt;</div>` to make sure it doesn't interpret
    `{message}` as HTML! This is the biggest danger, and might in fact be the case.
  - The POST format, which offhand seems like it's probably 8-bit clean (the HTTP request gives a
    length for the body so you don't need any delimeters or parsing at all to read it or pass it
    on.) If it turned out not to be clean, I'd check if JS `fetch()` properly escapes values on the
    way in, and the bits that take the POST body's bytes and validate/decode them into UTF-8.
  - Serde's JSON encoder, which almost certainly properly escapes the message on the way out.
  - JS's JSON decoder, which almost certainly properly unescapes the message on the way in.

* **The "send message" box is disabled while sending.**

  This prevents jamming the send button over and over. We also clear the text box when it has
  successfully sent to make it obvious the message is sent, preventing the user from sending it
  again.

  We could have just cleared the message in the first place instead of disabling, which would have
  had the same effect, but then if the message *failed* to send it would just get lost forever. This
  way you get another try on failure.

* **New messages do not show up immediately after sending.**

  Due to polling, new messages won't necessarily show up by themselves until the next poll. This
  can cause a noticeable delay between taking an action (hitting Enter to send a message) and
  seeing the result of that action. People tend to get very irritated and feel sad about apps that
  do this. Users are less likely to notice the delay in *other* peoples' messages, since the the
  user's awareness isn't laser focused on an action they just took.

  **If I had more time,** I would mitigate this by triggering a poll as soon as you send a message.
  It would require some frontend code restructuring so that NewMessage.svelte has access to kick off
  a poll, but ultimately is very doable.

  If there was a considerable network delay it would be worth mitigating further by showing a "fake"
  version of your message until the poll comes in and shows it. (This is a typical UI strategy since
  this is such a big problem.)

Other Possible Features
-----------------------

Other features and tasks that might be worth doing:

* Authentication, nicknames and non-anonymity. Not all message boards need these, and some actively
  discourage them, but it's a feature that should be explicitly thought about.
* Threading, if conversations become a thing.
* Multiple message boards in a single server.

Developing
----------

If you run the watch command, the server will automatically rebuild and restart when you change
relevant files:

```sh
cargo watch -x run
```

### Tests

To run the tests:

```sh
cargo test
```
