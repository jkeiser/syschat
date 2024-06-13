<script lang="ts">
    /** New message text the user has typed, which should be sent. */
    let newMessage = "";

    /** Whether we are currently sending a message */
    let sendingMessage = false;

    async function sendMessage() {
        if (newMessage == '') { return; } // Don't send empty messages
        if (sendingMessage) { throw 'Already sending a message'; }

        sendingMessage = true;
        try {
            // Send the message
            let response = await fetch('/messages', { method: 'POST', body: newMessage });
            if (!response.ok) { throw response.statusText; }

            // Clear the message once it's successfully sent
            newMessage = '';
        } finally {
            sendingMessage = false;
        }
    }
</script>

<style>
    .newmessage {
        display: flex;
        margin-bottom: 1em;
    }

    .newmessage input {
        flex: 1;
        margin-right: 0.5em;
    }

    .newmessage button {
        flex: 0;
    }
</style>

<form class="newmessage" class:disabled={sendingMessage} on:submit|preventDefault={sendMessage}>
<input type="text" bind:value={newMessage} disabled={sendingMessage} />
<button type="submit" disabled={sendingMessage}>Send</button>
</form>
