<template>

  <section class="bg-gray-800 text-center fixed bottom-0 top-0 left-0 h-16 md:h-screen">
    <div class="md:relative mx-auto lg:px-6" style="background-image: url('/img/bg-material.jpg'); height: 120px;">
      <img src="/img/ic-logo.png" class="absolute bottom-0 pb-3">
    </div>
    <div class="md:relative mx-auto pt-4 px-6">
      <ul class="list-reset flex flex-row md:flex-col text-center md:text-left">
        <li class="mr-3 flex-1">
          <router-link to="/" exact active-class="text-white bg-gray-900 md:font-bold" class="block py-1 md:py-3 pl-1 align-middle text-gray-400 no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <i class="fas fa-link pr-0 md:pr-3"></i><span class="pb-1 md:pb-0 text-xs md:text-base block md:inline-block">대시보드</span>
          </router-link>
        </li>
        <li class="mr-3 flex-1">
          <router-link to="/contents" active-class="text-white bg-gray-900 md:font-bold" class="block py-1 md:py-3 pl-1 align-middle text-gray-400 no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <i class="fas fa-link pr-0 md:pr-3"></i><span class="pb-1 md:pb-0 text-xs md:text-base block md:inline-block">도움말작성</span>
          </router-link>
        </li>
        <li class="mr-3 flex-1">
          <router-link to="/group" active-class="text-white bg-gray-900 md:font-bold" class="block py-1 md:py-3 pl-1 align-middle text-gray-400 no-underline rounded-md hover:bg-gray-700 hover:text-white"><!-- border-pink-600 -->
            <i class="fas fa-link pr-0 md:pr-3 text-pink-500"></i><span class="pb-1 md:pb-0 text-xs md:text-base block md:inline-block">도움말그룹</span>
          </router-link>
        </li>
        <li class="mr-3 flex-1">
          <router-link to="/history" active-class="text-white bg-gray-900 md:font-bold" class="block py-1 md:py-3 pl-1 align-middle text-gray-400 no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <i class="fas fa-link pr-0 md:pr-3"></i><span class="pb-1 md:pb-0 text-xs md:text-base block md:inline-block">변경이력</span>
          </router-link>
        </li>
        <li class="mr-3 flex-1">
          <a href="#" class="block py-1 md:py-3 pl-1 align-middle text-gray-400 no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <i class="fas fa-link pr-0 md:pr-3"></i><span class="pb-1 md:pb-0 text-xs md:text-base block md:inline-block">배포관리</span>
          </a>
        </li>
        <li class="mr-3 flex-1">
          <a href="#" class="block py-1 md:py-3 pl-1 align-middle text-gray-400 no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <i class="fas fa-link pr-0 md:pr-3"></i><span class="pb-1 md:pb-0 text-xs md:text-base block md:inline-block">FTP업로드&nbsp;&nbsp;<span class="sidebar-badge inline-flex items-center justify-center px-2 text-xs font-bold leading-none text-red-100 bg-red-600 rounded-full">pro</span></span>
          </a>
        </li>
        <li class="mr-3 flex-1">
          <a href="#" class="block py-1 md:py-3 pl-1 align-middle text-gray-400 no-underline rounded-md hover:bg-gray-700 hover:text-white">
            <i class="fas fa-link pr-0 md:pr-3"></i><span class="pb-1 md:pb-0 text-xs md:text-base block md:inline-block">공동편집자&nbsp;&nbsp;<span class="sidebar-badge inline-flex items-center justify-center px-2 text-xs font-bold leading-none text-red-100 bg-red-600 rounded-full">pro</span></span>
          </a>
        </li>
      </ul>
    </div>
    <div class="absolute bottom-0 w-full px-6" style="height: 36px;">
      <img src="/img/ic-maker.png">
    </div>
  </section>

</template>

<script>
export default {
  data() {
    return {
    };
  },
  methods: {
  },
  components: {
  },
};
</script>

<style scoped>
section { width: 12rem; }
.sidebar-badge { padding-top: .1rem; padding-bottom: .3rem; }
</style>
