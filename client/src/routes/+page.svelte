<script>
    import NewMessage from './NewMessage.svelte';
    import LiveMessages from './LiveMessages.svelte';
</script>

<style>
    .content {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .title {
        font-size: 2em;
        margin-bottom: 1em;
    }

    .messages {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>

<div class="content">
    <div class="title">Message Board</div>
    <div class="messages">
        <NewMessage />
        <LiveMessages />
    </div>
</div>
