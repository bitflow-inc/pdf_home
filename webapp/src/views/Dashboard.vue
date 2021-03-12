<template>

  <div class="w-full h-full">

    <div class="group-board p-6 grid grid-cols-4 gap-4">
      <dashboard-group-item v-for="item in group_list" v-bind:key="item.id"></dashboard-group-item>
    </div>

    <div class="px-6 pb-6 grid grid-cols-2 gap-4">

      <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
        <div class="flex justify-between bg-blue-300">
          <div class="mx-6 my-3 font-semibold text-white">변경이력</div>
          <div>
            <router-link to="/history">
              <button class="h-7 px-2 m-2 text-sm text-gray-7000 transition-colors duration-150 rounded-lg focus:shadow-outline hover:bg-gray-1000">더보기</button>
            </router-link>
          </div>
        </div>
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center">
              일시
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center">
              도움말그룹
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center">
              도움말
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center">
              변경자
            </th>
          </tr>
          </thead>

          <tbody class="bg-white divide-y divide-gray-200 text-xs">
          <tr>
            <td class="px-6 py-4 whitespace-nowrap">
              10월23일 13시
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div>국내주식</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div>[A3001] 입출금</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              관리자
            </td>
          </tr>

          <!-- More items... -->
          </tbody>
        </table>
      </div>


      <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
        <div class="flex justify-between bg-blue-300">
          <div class="mx-6 my-3 font-semibold text-white">미배포 변경파일</div>
          <div>
            <router-link to="/download">
              <button class="h-7 px-2 m-2 text-sm text-gray-7000 transition-colors duration-150 rounded-lg focus:shadow-outline hover:bg-gray-1000">더보기</button>
            </router-link>
          </div>
        </div>
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center">
              도움말그룹
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center">
              도움말ID
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center">
              제목
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center">
              파일명
            </th>
          </tr>
          </thead>

          <tbody class="bg-white divide-y divide-gray-200 text-xs">
          <tr>
            <td class="px-6 py-4 whitespace-nowrap">
              국내주식
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              A3001
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              [A3001] 입출금
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              A3001.PDF
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
import DashboardGroupItem from "../components/DashboardGroupItem"

export default {
  name: "dashboard",
  components: {
    DashboardGroupItem,
  },
  data() {
    return {
      group_list:  [ { id: "domestic", name: "국내주식", order_no: 1, tree: "" },
        { id: "foreign", name: "해외주식", order_no: 2, tree: "" },
        { id: "global", name: "세계주식", order_no: 3, tree: "" }, ]
    }
  },
  methods: {
  },
  mounted() {
  }

};
</script>

<style>
.group-board > div:first-child { --tw-bg-opacity: 1; background-color: rgba(209,213,219,var(--tw-bg-opacity)); }
.group-board > div:nth-child(2) { --tw-bg-opacity: 1; background-color: rgba(229, 231, 235, var(--tw-bg-opacity)); }
.group-board > div:nth-child(3) { --tw-bg-opacity: 1; background-color: rgba(243, 244, 246, var(--tw-bg-opacity)); }
</style>