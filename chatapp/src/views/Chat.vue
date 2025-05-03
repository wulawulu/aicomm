<template>
  <div class="chat">
    <Sidebar />
    <div class="chat-main">
      <MessageList />
      <MessageSend />
    </div>
  </div>
</template>

<script>
import Sidebar from '../components/Sidebar.vue';
import MessageList from '../components/MessageList.vue';
import MessageSend from '../components/MessageSend.vue';

export default {
  components: {
    Sidebar,
    MessageList,
    MessageSend,
  },
};
</script>

<style scoped>
.chat {
  display: flex;
  height: 100vh; /* Full height to fit the screen */
}

.chat-main {
  display: flex;
  flex-direction: column;
  flex: 1;
  position: relative;
}

.chat-main > .message-list {
  flex: 1;
  overflow-y: auto; /* Scrollable message list */
}

.chat-main > .message-send {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
}
</style>
