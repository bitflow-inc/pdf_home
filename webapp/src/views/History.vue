<template>

  <div class="w-full h-full">
    <div class="header relative mx-auto px-6 bg-white h-9 pt-1 shadow flex justify-between">
      <div class="pt-0.5 font-semibold">변경이력</div>
    </div>

    <div class="p-6 overflow-y-auto overflow-x-hidden">
      <!-- This example requires Tailwind CSS v2.0+ -->
      <div class="flex flex-col">
        <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
          <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
            <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                <tr>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    순번
                  </th>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    변경일시
                  </th>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    변경자
                  </th>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    도움말그룹
                  </th>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    도움말ID
                  </th>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    제목
                  </th>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    변경사유
                  </th>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    파일명
                  </th>
                  <th scope="col" class="px-6 py-3 text-center text-xs font-medium text-gray-500 uppercase tracking-wider">
                    다운로드
                  </th>
                </tr>
                </thead>

                <tbody class="bg-white divide-y divide-gray-200">
                <tr v-for="item in list" v-bind:key="item.id" class="text-sm">
                  <td class="px-6 py-4 whitespace-nowrap">
                    {{item.id}}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    {{item.upd_dt}}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    {{item.upd_author}}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    {{item.group_id}}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    {{item.content_id}}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    {{item.title}}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    {{item.comment}}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-right">
                    <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                    {{item.filename}}
                    </span>
                  </td>
                  <td class="px-6 whitespace-nowrap">
                    <a href="#">
                      <img src="/img/ic-html.png">
                    </a>
                  </td>
                </tr>

                <!-- More items... -->
                </tbody>
              </table>
            </div>
          </div>
        </div>
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
        , upd_author: "관리자", upd_dt: "1월27일 13시", comment: "신규등록" },
        { id: 410, group_id: "domestic", content_id: "A3003", content_type: "HTML", title: "[A3003] 제목", filename: "A3003.pdf"
          , upd_author: "관리자", upd_dt: "1월27일 13시", comment: "신규등록" },],
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
