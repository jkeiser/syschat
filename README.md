Syschat is an ultra-simple anonymous message board.

Quickstart
----------

Run this (prerequisites: [Rust](https://www.rust-lang.org/tools/install) and [Node](https://nodejs.org/en/download/)):

  ```sh
  cargo run
  ```

And open your browser to [http://localhost:3000](http://localhost:3000).

Tech Decisions
--------------

These don't affect features directly, but do affect the development and operational experience.

* **Svelte for frontend.**

  I don't have a super strong preference on frontend frameworks, though I've enjoyed Vue and Svelte
  more than React. Vue would be a better choice in some ways because System Initiative uses it, and
  it's important to pick techs your team will be comfortable with. But I really like the compiled
  approach Svelte takes and wanted to explore it a little more :)

  This decision did cost me a couple of hours on what seems like a bug in vite+svelte's live
  reloading experience (but could definitely still be something I don't understand, I'm not an old
  hand at Svelte!). Compiling statically and using the served files instead of the live debugging
  experience was how I got around that.

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
  
  **If I had more time,** I would compile the static files into the executable rather than require
  them to be in a specific place on the drive. This would vastly simplify the deployment of the
  server (just ship the executable), and remove its dependency on the filesystem as well,
  simplifying management further.

Features / Design
-----------------

These are some of the major features and design decisions, tradeoffs, and things that I might have
done if I had more time.

* **The server stores all message history as an array in memory.**

  This is in the spec, but the real reason is limited dev time: getting persistence right (and
  reliable) takes time.

  The lack of a limit on message history means the process will eventually run out of memory
  and crash. This is mitigated somewhat since the server will successfully start fresh (with no
  messages) next time. We already accept that the server will lose all messages on restart due to
  the in-memory design, and this is just one more way that can happen.

  **If I had more time,** I would persist messages and only keep recent messages in memory, allowing
  a much larger message limit (theoretically, everything runs out, it just depends how long you
  wait).
  
  This could also be mitigated without adding persistence by implementing a limit and destroying the
  oldest messages when it would grow above that limit.

* **The client loads all messages on startup.**

  This allows the user to see what's been happening in the past and is important user experience.

  The alternative design would be to show only new messages, but that means knowing nothing about
  what anyone else has ever said, possibly repeating stuff, and having nothing to respond to.

* **The client updates live by pulling all messages from the server once per second.**

  Live updates for new messages is more engaging and less annoying to the user than hitting reload
  over and over.

  Retrieving all the messages each second will become a network and compute hog as the number of
  messages grows, and it is completely unnecessary to re-send the same messages over and over
  again: the client already knows about them!

  **If I had more time,** I would serve live message updates over a web socket and only listen for
  new messages after the initial load.

  A lesser solution would be to poll, but ask for only new messages since the last poll. This can
  still be a lot more expensive than a web socket, as you have to do an entire back-and-forth TCP
  connection and HTTP GET each second, headers included. A web socket only needs a periodic
  heartbeat to keep the connection alive.

  This could also be mitigated by implementing pagination and polling for all visible messages, but
  on top of the TCP and HTTP overhead, constantly reloading even 10 messages can add up if there are
  a lot of clients.

* **Newest messages are at the top.**

  This allows new messages to be close to where the eye is.
  
  The drawback is that generally computers prefer if you add things to the end of lists, and
  frontend frameworks are no exception. I'm a little worried that Svelte might completely obliterate
  and recreate the entire DOM for all existing messages each time a single new message comes in. I
  haven't really researched how Svelte handles changes to lists, but I can imagine frameworks
  comparing old and new lists element by element, and when the second list is just shifted by one,
  all elements will compare different! (Svelte probably worried about this too, so maybe it's not
  an issue.)

  It doesn't help that due to the polling pulling every message, the new messages don't have the
  same identity as their old counterparts.

  **If I had more time,** I would have investigated Svelte to see whether this is causing massive
  DOM upheaval. Then decided what to do about that, if anything.

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

  This prevents jamming the send button over and over.

* **New messages do not show up immediately after sending.**

  Due to polling, new messages won't necessarily show up by themselves until the next poll. This
  can cause a noticeable delay between taking an action (hitting Enter to send a message) and
  seeing the result of that action. People tend to get very irritated and feel sad about apps that
  do this. Users are less likely to notice the delay in *other* peoples' messages, since the the
  user's awareness isn't laser focused on an action they just took.

  **If I had more time,** I would mitigate this by triggering a poll as soon as you send a message.
  If there was a considerable network delay it would be worth mitigating further by showing a "fake"
  version of your message until the poll comes in and shows it. (This is a typical UI strategy since
  this is such a big problem.)

**If I had more time,** I would also look into these features at the very least:

* UI polish. The Vite+Svelte live reloading things cost me a lot of polish time! I'm not sad about
  the current UI, and think it's pretty reasonable for a place for people to leave "notes." One
  thing I'd like to see is new messages to "pop" more, briefly showing bolded or outlined or colored
  for a bit so you notice them more.
* Authentication, nicknames and non-anonymity. Not all message boards need these, and some actively
  discourage them, but it's a feature that should be explicitly thought about.
* Threading, if conversations become a thing.
* Multiple message boards in a single server.
* Everything noted in the previous section.

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
