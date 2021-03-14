<template>

  <section class="bg-gray-800 text-center fixed bottom-0 top-0 left-0 h-16 md:h-screen">
    <div class="md:relative mx-auto lg:px-6" style="background-image: url('/img/bg-material.jpg'); height: 120px;">
      <img src="/img/ic-logo.png" class="absolute bottom-0 pb-3">
    </div>
    <div class="md:relative mx-auto pt-4 px-4">
      <ul class="list-reset flex flex-row flex-col text-center text-left text-gray-400">
        <li class="mb-1 flex-1">
          <router-link to="/" exact active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <monitor-dashboard></monitor-dashboard><span class="absolute top-2 left-11">대시보드</span>
          </router-link>
        </li>
        <li class="mb-1 flex-1">
          <router-link to="/contents" active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <pencil-plus></pencil-plus><span class="absolute top-2 left-11">도움말작성</span>
          </router-link>
        </li>
        <li class="mb-1 flex-1">
          <router-link to="/group" active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white"><!-- border-pink-600 -->
            <file-document-multiple-outline></file-document-multiple-outline><span class="absolute top-2 left-11">도움말그룹</span>
          </router-link>
        </li>
        <li class="mb-1 flex-1">
          <router-link to="/history" active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <order-numeric-descending></order-numeric-descending><span class="absolute top-2 left-11">변경이력</span>
          </router-link>
        </li>
        <li class="mb-1 flex-1">
          <router-link to="/download" active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <parachute></parachute><span class="absolute top-2 left-11">다운로드</span>
          </router-link>
        </li>
        <li class="mb-1 flex-1">
          <router-link to="#" active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <floor-plan></floor-plan><span class="absolute top-2 left-11">템플릿관리</span>
            <span class="absolute right-0 top-0 sidebar-badge inline-flex items-center justify-center px-1.5 text-xs font-bold leading-none text-red-100 bg-red-600 rounded-full">pro</span>
          </router-link>
        </li>
        <li class="mb-1 flex-1">
          <router-link to="#" active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <server></server><span class="absolute top-2 left-11">셀프호스팅</span>
            <span class="absolute right-0 top-0 sidebar-badge inline-flex items-center justify-center px-1.5 text-xs font-bold leading-none text-red-100 bg-red-600 rounded-full">pro</span>
          </router-link>
        </li>
        <li class="mb-1 flex-1">
          <router-link to="#" active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <folder-upload-outline></folder-upload-outline><span class="absolute top-2 left-11">FTP업로드</span>
            <span class="absolute right-0 top-0 sidebar-badge inline-flex items-center justify-center px-1.5 text-xs font-bold leading-none text-red-100 bg-red-600 rounded-full">pro</span>
          </router-link>
        </li>
        <li class="flex-1">
          <router-link to="#" active-class="text-white bg-gray-900 font-bold" class="relative block py-2 pl-1 align-middle no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <account-supervisor></account-supervisor><span class="absolute top-2 left-11">공동편집자</span>
            <span class="absolute right-0 top-0 sidebar-badge inline-flex items-center justify-center px-1.5 text-xs font-bold leading-none text-red-100 bg-red-600 rounded-full">pro</span>
          </router-link>
        </li>
      </ul>
    </div>
    <div class="absolute bottom-0 w-full px-6" style="height: 36px;">
      <img src="/img/ic-maker.png">
    </div>
  </section>

</template>

<script>
import MonitorDashboard from 'vue-material-design-icons/MonitorDashboard.vue';
import PencilPlus from 'vue-material-design-icons/PencilPlus.vue';
import Parachute from 'vue-material-design-icons/Parachute.vue';
import Server from 'vue-material-design-icons/Server.vue';
import FloorPlan from 'vue-material-design-icons/FloorPlan.vue';
import FolderUploadOutline from 'vue-material-design-icons/FolderUploadOutline.vue';
import AccountSupervisor from 'vue-material-design-icons/AccountSupervisor.vue';
import FileDocumentMultipleOutline from 'vue-material-design-icons/FileDocumentMultipleOutline.vue';
import OrderNumericDescending from 'vue-material-design-icons/OrderNumericDescending.vue';

export default {
  data() {
    return {
    };
  },
  methods: {
  },
  components: {
    MonitorDashboard,
    PencilPlus,
    FloorPlan,
    Parachute,
    Server,
    AccountSupervisor,
    FolderUploadOutline,
    FileDocumentMultipleOutline,
    OrderNumericDescending,
  },
};
</script>

<style scoped>
section { width: 12rem; }
.sidebar-badge { padding-top: .1rem; padding-bottom: .3rem; }
.material-design-icon svg { margin-left: 6px; }
</style>
