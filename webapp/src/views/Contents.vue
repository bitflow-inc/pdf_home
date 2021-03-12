<template>

  <div class="w-full h-full">
    <div class="header relative mx-auto px-6 bg-white h-9 pt-1 shadow flex justify-between">
      <div class="pt-0.5 font-semibold text-gray-700">도움말작성</div>
      <div>
        <select id="id-group" ref="id-group" class="px-4 py-0.5 w-72 rounded border-2">
          <option value="">도움말그룹1</option>
          <option value="1">도움말그룹1</option>
          <option value="2">도움말그룹1</option>
        </select>
      </div>
      <div>
        <button class="bg-transparent hover:bg-blue-500 text-blue-700 hover:text-white px-2 border border-blue-500 hover:border-transparent rounded h-7">
          취소
        </button>
        <button class="bg-blue-500 hover:bg-blue-700 text-white px-2 rounded h-7">PDF업로드</button>
        <button class="bg-blue-500 hover:bg-blue-700 text-white px-2 rounded h-7">저장</button>
      </div>
    </div>

    <div class="relative h-full">
      <section class="fixed left-50 top-9 bg-gray-300 text-center h-full w-72">
      </section>
      <div class="ml-72 p-6 overflow-y-auto">
        <froala :tag="'textarea'" :config="config" v-model="model"></froala>
      </div>
    </div>
  </div>

</template>
<script>
//import VueFroala from 'vue-froala-wysiwyg';

export default {
  name: 'contents',
  data () {
    return {
      config: {
        events: {
          'froalaEditor.initialized': function () {
            // console.log('initialized')
          }
        }
      },
      model: 'Edit Your Content Here!'
    }
  }
}
</script>

<style>
#id-group { border: 1px solid #333; }
.top-nav { margin-left: 12rem; width: calc(100% - 12rem); }
</style>
