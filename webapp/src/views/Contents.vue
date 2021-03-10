<template>

  <div class="w-full h-full relative">
    <div class="top-nav fixed top-0 left-0 w-full mx-auto lg:px-6 bg-white h-12 pt-1.5 z-30 shadow flex justify-between">
      <div>
        <label>도움말작성&nbsp;&nbsp;</label>
        <select id="id-group" ref="id-group" class="px-4 py-1.5 rounded border-2">
          <option value="">도움말그룹1</option>
          <option value="1">도움말그룹1</option>
          <option value="2">도움말그룹1</option>
        </select>
      </div>
      <div>
        <button class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-1 px-2 border border-blue-500 hover:border-transparent rounded h-9">
          취소
        </button>
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded h-9">PDF업로드</button>
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded h-9">HTML작성</button>
      </div>
    </div>
    <div class="relative h-full pt-12">
      <section class="fixed left-50 top-12 bg-gray-300 text-center h-full w-72">
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
