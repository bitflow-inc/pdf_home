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
input[type=text] { height: 2rem; padding: .5rem; border-width: 0.1rem; }
input[type=text]:read-only { background-color: transparent; border-width: 0; }
input[type=text]:focus { background-color: white; }
button { font-size: 0.9rem; background-color: white; --tw-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1),0 1px 2px 0 rgba(0, 0, 0, 0.06);
  box-shadow: var(--tw-ring-offset-shadow,0 0 #0000),var(--tw-ring-shadow,0 0 #0000),var(--tw-shadow);
}
button + button { margin-left: 6px; }
.header { background-image: url('/img/bg-title.png'); background-repeat: no-repeat; }
#content-area { width: calc(100% - 12rem); margin-left: 12rem; }
</style>
