<template>

  <div class="w-full h-full">
    <div class="header relative mx-auto px-4 bg-white h-9 pt-1 shadow flex justify-between">
      <div class="pt-0.5 font-semibold text-gray-700">다운로드</div>
      <div>
        <button class="bg-transparent hover:bg-blue-500 text-blue-700 hover:text-white px-2 border border-blue-500 hover:border-transparent rounded h-7">
          전체파일
        </button>
        <button class="bg-blue-500 hover:bg-blue-700 text-white px-2 rounded h-7">선택파일</button>
        <button class="bg-blue-500 hover:bg-blue-700 text-white px-2 rounded h-7">변경파일전체</button>
      </div>
    </div>

    <div class="box-content w-full" style="background-image: url('/img/bg-menu.jpg'); background-position-y: -168px;">
    </div>

    <div class="p-6 overflow-y-auto overflow-x-hidden grid grid-cols-5 gap-4">

      <div class="text-sm shadow overflow-hidden border-b border-gray-200 sm:rounded-lg col-span-3">

        <div class="flex justify-between bg-blue-300">
          <div class="mx-6 my-2 font-semibold text-white">미배포 파일</div>
        </div>

        <table class="min-w-full divide-y divide-gray-200 text-xs">
          <thead class="bg-gray-50">
          <tr>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
             변경일시
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              변경자
            </th>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              도움말그룹
            </th>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              제목
            </th>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              사유
            </th>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              파일명
            </th>
          </tr>
          </thead>

          <tbody class="bg-white divide-y divide-gray-200 text-xs">
          <tr v-for="item in list" v-bind:key="item.id" class="text-sm">
            <td class="px-4 py-4 whitespace-nowrap text-center">
              {{item.upd_dt}}
            </td>
            <td class="px-4 py-4 whitespace-nowrap text-center">
              {{item.upd_author}}
            </td>
            <td class="px-4 py-4 whitespace-nowrap text-center">
              {{item.group_id}}
            </td>
            <td class="px-4 py-4 whitespace-nowrap text-center">
              {{item.title}}
            </td>
            <td class="px-4 py-4 whitespace-nowrap text-center" v-bind:title="item.comment">
              {{item.comment}}
            </td>
            <td class="px-4 py-4 whitespace-nowrap text-center">
              <span class="px-2 inline-flex leading-5 font-semibold rounded-full bg-green-100 text-green-800">
              {{item.filename}}
              </span>
            </td>
          </tr>

          <!-- More items... -->
          </tbody>
        </table>
      </div>

      <div class="text-sm shadow overflow-hidden border-b border-gray-200 sm:rounded-lg col-span-2">

        <div class="flex justify-between bg-blue-300">
          <div class="mx-6 my-2 font-semibold text-white">배포이력</div>
        </div>

        <table class="min-w-full divide-y divide-gray-200 text-xs">
          <thead class="bg-gray-50">
          <tr>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              일시
            </th>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              파일명
            </th>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              사유
            </th>
            <th scope="col" class="px-4 py-3 text-center font-medium text-gray-500 uppercase tracking-wider">
              배포자
            </th>
          </tr>
          </thead>

          <tbody class="bg-white divide-y divide-gray-200 text-xs">
          <tr v-for="item in list" v-bind:key="item.id" class="text-sm">
            <td class="px-4 py-4 whitespace-nowrap text-center">
              {{item.upd_dt}}
            </td>
            <td class="px-4 py-4 whitespace-nowrap text-center">
              <span class="px-2 inline-flex leading-5 font-semibold rounded-full bg-green-100 text-green-800">
              {{item.filename}}
              </span>
            </td>
            <td class="px-4 py-4 whitespace-nowrap text-center" v-bind:title="item.comment">
              {{item.comment}}
            </td>
            <td class="px-4 py-4 whitespace-nowrap text-center">
              {{item.upd_author}}
            </td>
          </tr>

          <!-- More items... -->
          </tbody>
        </table>
      </div>

    </div>
  </div>

</template>

<script>
export default {
  name: "history-list",
  components: {
  },
  data() {
    return {
      list: [ { id: 411, group_id: "domestic", content_id: "A3003", content_type: "PDF", title: "[A3003] 제목", filename: "A3003.pdf"
        , upd_author: "관리자", upd_dt: "1월27일 13시", comment: "신규등록", method: "MOD", realpath: "" },
        { id: 410, group_id: "domestic", content_id: "A3003", content_type: "HTML", title: "[A3003] 제목", filename: "A3003.pdf"
          , upd_author: "관리자", upd_dt: "1월27일 13시", comment: "신규등록", method: "ADD", realpath: "" } ],
    }
  },
  methods: {
    requestImages() {
      window.fetch('/api/v1/images')
          .then((response) => {
            if(response.ok){
              return response.json();
            }
            throw new Error('Network response was not ok');
          })
          .then((json) => {
            this.items = json;
          })
          .catch(() => {
            // console.log(error);
          })
    },
  },
  mounted() {
  }

};
</script>

<style scoped>
.box-content { height: 84px; opacity: 0.5; }
td { font-size: .75rem; line-height: 1rem; }
</style>
