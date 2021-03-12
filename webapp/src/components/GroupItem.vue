<template>

  <div class="relative row-wrapper px-4 py-2 border-4 border-solid border-gray-200 shadow rounded-lg h-64 bg-white">
    <div>
      <label for="name" class="block text-sm font-medium text-gray-700 float-left">그룹명</label>&nbsp;<span class="hidden text-xs float-right mt-0.5 text-red-500">그룹명을 입력해주세요</span>
      <input type="text" name="name" id="name" ref="name" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 focus:bg-white block w-full h-64sm:text-sm border-gray-200 rounded-md h-8 p-2" v-model="group_name" placeholder="한글, 영문 및 숫자" maxlength="50"/>
    </div>
    <div>
      <label for="id" class="block text-sm font-medium text-gray-700 float-left">웹주소</label>&nbsp;<span class="hidden text-xs float-right mt-0.5 text-red-500">웹주소를 입력해주세요</span>
      <input type="text" name="id" id="id" ref="id" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full h-64sm:text-sm border-gray-200 rounded-md" @keypress="isAlphaNumericPress($event)" @change="isAlphaNumericUp($event)" v-model="group_id" placeholder="영소문자 또는 숫자" maxlength="20"/>
    </div>
    <div>
      <label for="order_no" class="block text-sm font-medium text-gray-700 float-left">정렬순서</label>&nbsp;<span class="hidden text-xs float-right mt-0.5 text-red-500">정렬순서를 입력해주세요</span>
      <input type="text" name="order_no" id="order_no" ref="order_no" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full h-64sm:text-sm border-gray-200 rounded-md" v-model="group_order_no" placeholder="0~255까지 숫자" maxlength="3"/>
    </div>
    <div class="flex justify-end" v-if="isEmpty">
      <button class="hover:bg-blue-500 text-blue-700 hover:text-white px-2 border border-blue-500 hover:border-transparent rounded h-7" @click="cancelWrite()">
        취소
      </button>
      <button class="bg-blue-500 hover:bg-blue-700 text-white px-2 rounded h-7" @click="saveGroup()">저장</button>
    </div>
    <div class="flex justify-end" v-else>
      <button class="hover:bg-blue-500 text-blue-700 hover:text-white px-2 border border-blue-500 hover:border-transparent rounded h-7" @click="cancelModify()">
        삭제
      </button>
      <button class="bg-blue-500 hover:bg-blue-700 text-white px-2 rounded h-7" @click="goModify()">수정</button>
    </div>
    <div v-if="!isEmpty" class="absolute right-2 top-0 sidebar-badge inline-flex items-center justify-center px-2 py-2 text-xs font-bold leading-none text-red-100 bg-red-600 rounded-full">119</div>
  </div>

</template>

<script>
export default {
  props: {
    isEmpty: Boolean,
    id: String,
    name: String,
    order_no: Number,
    tree: String,
  },
  data() {
    return {
      group_id: null,
      group_name: null,
      group_order_no: null,
      group_tree: null,
    };
  },
  methods: {
    cancelWrite: function() {
      if (this.isEmpty===true) {
        this.$destroy();
        this.$el.parentNode.removeChild(this.$el);
      } else {
        this.$refs.name.setAttribute("readonly", true);
        this.$refs.order_no.setAttribute("readonly", true);
      }
    },
    cancelModify: function() {
      this.$refs.name.setAttribute("readonly", true);
      this.$refs.order_no.setAttribute("readonly", true);
    },
    checkValidation: function() {

      return true;
    },
    saveGroup: function() {
      if (this.checkValidation()==true) {
          return true;
      }
    },
    goModify: function() {
      this.$refs.name.removeAttribute("readonly");
      this.$refs.order_no.removeAttribute("readonly");
      this.$refs.name.focus();
    },
    isAlphaNumericPress: function(e) {
      let char = String.fromCharCode(e.keyCode); // Get the character
      if(/^[a-z0-9]+$/.test(char)) return true; // Match with regex
      else e.preventDefault(); // If not match, don't add to input text
    },
    isAlphaNumericUp: function(e) {
      if (this.group_id!==null && !(e.keyCode>=37 && e.keyCode<=40)) {
        // let changed = this.group_id.replace(/[ㄱ-ㅎㅏ-ㅣ가-힣]/g, '');
        // console.log("changed " + changed);
        // this.group_id = "";
        this.group_id = e.target.value.replace(/[ㄱ-ㅎㅏ-ㅣ가-힣]/g, '');
      }
    }
  },
  mounted() {
    console.log('isempty ' + this.isEmpty);
    if (this.isEmpty===null || this.isEmpty===false) {
      this.group_id = this.id;
      this.group_name = this.name;
      this.group_order_no = this.order_no;
      this.group_tree = this.tree;
      this.$refs.name.setAttribute("readonly", true);
      this.$refs.id.setAttribute("readonly", true);
      this.$refs.order_no.setAttribute("readonly", true);
    }
    console.log("this.group_id " + this.group_id);
  }
};
</script>

<style scoped>
.row-wrapper { background-image: url('/img/bg-memo.jpg'); }
.row-wrapper > div + div { margin-top: 0.5rem; }
.row-wrapper > div + div.flex { margin-top: 1rem; }
</style>
