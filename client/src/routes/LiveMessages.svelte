<script lang="ts">
    type Message = { timestamp?: string; message: string; };
    let messages: Message[] = [];

    // Poll the server for new messages every second
    async function updateMessages() {
        let response = await fetch(`/messages?first_message_id=${messages.length}`);
        if (!response.ok) { throw response.statusText; }
        const newMessages = await response.json();
        if (newMessages.length > 0) {
            console.log(`Fetched ${messages.length} new messages.`)
            messages = [...newMessages.reverse(), ...messages]; // Most recent first
        }
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
