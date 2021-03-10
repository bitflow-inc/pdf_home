<template>

  <div class="flex flex-row flex-wrap">

    <!--Sidebar-->
    <sidebar></sidebar>

    <!--Main Content-->
    <div id="content-area" class="bg-gray-100">
      <router-view></router-view>
    </div>

  </div>

</template>
<script>
import Sidebar from'./components/Sidebar.vue'
export default {
  name: "dashboard-page",
  components: {
    Sidebar,
  },
  data() {
    return {
    }
  }
};
</script>

<style>
#content-area { width: calc(100% - 12rem); margin-left: 12rem; }
button + button { margin-left: 6px; }
</style>
