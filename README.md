Syschat is an ultra-simple anonymous message board. The message board client is written in Svelte.
Clients connect to a backend HTTP API server, written in Rust/axum, which stores and relays messages
to all clients.

This project was built with a strict 4 hour time limit, so we had to make tradeoffs, minimizing
features so that the time can be used to make those features as polished and solid as possible.

Syschat is a simple *message feed* where short notes and thoughts can be left and seen in real
time.

- Conversation is a non-goal. 
- Messages show in real time.
  - This allows a feeling of emotional connection to the public leaving messages, increasing time spent.
  - Having to press a button or reload the page to see new messages would make the message board feel
    static and unchanging.
  - Users can see new messages just by watching, increasing the time they will spend watching the screen.

- Messages are anonymous.
  given the general tenor of conversations between anonymous users, but authentication . Conversations in the feed are possible.
Long but where
conversation is a non-goal.

* Message history shows up when the chatroom is loaded. To do otherwise means you can't see what
  other people in the past thought, which defeats the purposeenter the conversation
* Messages are one-offs, not conversations. There is no way to tell if multiple messages come from the
  same or different users.
  - Visually distinguishing messages from the *same user* from other messages would allow for
    conversations. This could be done by:
    1. Allowing users to provide a nickname. This would 
        2. Assigning a different color and/or style to each user. Since there are limited styles  
    3. Highlighting all messages by the same user when you hover over one of their messages. This would
       help mitigate the limited number of possible colors
  - Allowing users to log in would allow 
   (like a nickname or color) would allow
    for better anonymous It would be better to have a way to tell whether multiple messages were from the same person (or
    client). This could be done fairly quickly by assigning each connection a globally unique ID and
    marking messages with that ID. Then the client could assign colors to clients that send multiple
    messages so that it is clear they came from the same person.
    at least let you tell which messages were from the same user.
  Had we more time we would have added user 

, as this
  would be a rather annoying user experience: you should be able to tell just by looking if there are
  new messages, and should not be left to wonder.

Quickstart
----------



- The chatroom can be entered by going to the server's root URL, e.g. `http://localhost:8080/`.
- When first entering the chatroom, the full message history is loaded and displayed so the conversation can continue.
- Send Message:
    - A Send Message box is displayed, along with a Send button.
    - When the user presses Enter or clicks the Send button, the message is sent to the message server.
    - While 
- When a user enters the chatroom, the full message history is displayed.
- When a user sends a message, it is displayed instantly

populates new messages
from all clients as soon as they come. keeps a live connection
to a Rust axum backend http server, relaying live messages

* The frontend is a single-page application written in Svelte.
* It connects to an axum HTTP server
written in Rust, which keeps track of and retrieves messages.
* Message storage and retrieval:
* Messages are served by a Rust axum HTTP server.
* Messages are stored in-memory. When the server dies,
* There is no concurrency.