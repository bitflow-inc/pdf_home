<template>

  <div class="w-full h-full">

    <div class="header relative mx-auto px-6 bg-white h-9 pt-1 shadow flex justify-between">
      <div class="pt-0.5 font-semibold text-gray-700">도움말그룹</div>
    </div>

    <div class="box-content w-full" style="background-image: url('/img/bg-menu.jpg');">
    </div>

    <div ref="groupList" class="p-6 overflow-y-auto grid grid-cols-4 gap-4" v-for="item in list" v-bind:key="item.id">

      <group-item v-bind="item"></group-item>

      <div class="row-wrapper border-4 border-dashed border-gray-200 rounded-lg h-64 bg-white">
        <div class="bg-white h-full grid justify-items-center place-items-center">
          <img id="btn-add" ref="btn-add" src="/img/ic-add.jpg" @click="onBtnAddClicked(this)">
        </div>
      </div>

    </div>

  </div>

</template>
<script>
import GroupItem from "../components/GroupItem"
import Vue from "vue"

export default {
  name: "image-list",
  components: {
    GroupItem,
  },
  data() {
    return {
      list: [ { id: "domestic", name: "국내주식", order_no: 1, tree: "" } ]
    }
  },
  methods: {
    goUpdate: function() {

    },
    onBtnAddClicked: function() {
      let Item = Vue.extend(GroupItem);
      let item = new Item({
        propsData: {
          isEmpty: true
        }
      });
      item.$mount();
      this.$refs.groupList[this.$refs.groupList.length-1].append(item.$el);
    }
  },
  mounted() {
    // let Item = Vue.extend(GroupItem);
    // let item = new Item();
    // item.$mount();
    // this.$refs.groupList.insertBefore(item.$el, this.$refs.groupList.lastChild);
  }

};
</script>

<style scoped>
label { }
.box-content { height: 84px; opacity: 0.5; }
.row-wrapper { background-image: url('/img/bg-memo.jpg'); }
.row-wrapper > div + div { margin-top: 0.5rem; }
.row-wrapper > div + div.flex { margin-top: 1rem; }
#btn-add { max-width: 50%; opacity: 0.3; cursor: pointer; }
</style>