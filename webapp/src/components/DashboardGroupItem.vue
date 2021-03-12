<template>

  <div class="relative row-wrapper px-4 py-2 border-1 border-solid border-gray-200 shadow rounded-lg h-50 bg-white">
    <div class="grid grid-cols-2 text-sm">
      <div>도움말그룹</div><div>{{name}}</div>
      <div>도움말수</div><div>112</div>
      <div>최근변경</div><div>10월27일 13시</div>
    </div>
  </div>

</template>

<script>
export default {
  props: {
    id: String,
    name: String,
    order_no: Number,
    tree: String,
  },
  data() {
    return {
    };
  },
  methods: {
  },
  mounted() {
    console.log('name ' + this.name);
  }
};
</script>

<style>
.row-wrapper > div + div { margin-top: 0.5rem; }
.row-wrapper > div + div.flex { margin-top: 1rem; }
</style>
