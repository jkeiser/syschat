<script lang="ts">
    type Message = { timestamp?: string; message: string; };
    let messages: Message[] = [];

    // Poll the server for new messages every second
    async function updateMessages() {
        let response = await fetch(`/messages`); // ?first_message_id=${messages.length}
        if (!response.ok) { throw response.statusText; }
        messages = await response.json();
        console.log(`Fetched ${messages.length} messages.`)
    }

    setInterval(updateMessages, 1000);
</script>

<style>
    .message {
        margin: 0.5em;
        padding: 0.5em;
        border: 1px solid black;
    }
</style>

{#each messages as message}
    <div class="message">{message.message}</div>
{/each}
