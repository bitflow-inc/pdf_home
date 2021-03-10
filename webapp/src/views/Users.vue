<template>

  <div class="w-full h-full">
    <div class="md:relative mx-auto lg:px-6 bg-white h-12 pt-1.5 shadow flex justify-between">
      <div>사용자관리</div>
      <div>
        <button class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-1 px-2 border border-blue-500 hover:border-transparent rounded h-9">
          취소
        </button>
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded h-9">확인</button>
      </div>
    </div>
    <div class="p-6 overflow-y-auto grid grid-cols-4 gap-4">
      <div class="border-4 border-solid border-gray-200 rounded-lg h-64 bg-white">
        <p>김헌영</p>
        <p>admin</p>
        <p>#@!@#!@#@$</p>
      </div>
      <div class="border-4 border-solid border-gray-200 rounded-lg h-64 bg-white"></div>
      <div class="border-4 border-solid border-gray-200 rounded-lg h-64 bg-white"></div>
      <div class="border-4 border-solid border-gray-200 rounded-lg h-64 bg-white"></div>
      <div class="border-4 border-dashed border-gray-200 rounded-lg h-64 bg-white grid justify-items-center place-items-center">
        <img id="btn-add" src="/img/ic-add.jpg">
      </div>
    </div>
  </div>

</template>
<script>
export default {
  name: "image-list",
  components: {
  },
  data() {
    return {
    }
  },
  methods: {

  },
  mounted() {
  }

};
</script>

<style scoped>
#btn-add { max-width: 50%; opacity: 0.3; cursor: pointer; }
</style>