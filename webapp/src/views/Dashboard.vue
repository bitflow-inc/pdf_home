<template>

  <div class="w-full h-full">
    <div class="p-6 overflow-y-auto overflow-x-hidden">
      <!-- This example requires Tailwind CSS v2.0+ -->
      <div class="flex flex-col">
        <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
          <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
            <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                <tr>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Name
                  </th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Title
                  </th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Role
                  </th>
                  <th scope="col" class="relative px-6 py-3">
                    <span class="sr-only">Edit</span>
                  </th>
                </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex items-center">
                      <div class="flex-shrink-0 h-10 w-10">
                        <img class="h-10 w-10 rounded-full" src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&amp;ixid=eyJhcHBfaWQiOjEyMDd9&amp;auto=format&amp;fit=facearea&amp;facepad=4&amp;w=256&amp;h=256&amp;q=60" alt="">
                      </div>
                      <div class="ml-4">
                        <div class="text-sm font-medium text-gray-900">
                          Jane Cooper
                        </div>
                        <div class="text-sm text-gray-500">
                          jane.cooper@example.com
                        </div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="text-sm text-gray-900">Regional Paradigm Technician</div>
                    <div class="text-sm text-gray-500">Optimization</div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                  Active
                </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    Admin
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <a href="#" class="text-indigo-600 hover:text-indigo-900">Edit</a>
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
import 'whatwg-fetch'

export default {
  name: "image-list",
  components: {
  },
  data() {
    return {
      ctx: null,
      canvas: null,
      showingPopup: false,
      selectionMode: false,
      startPosition: {
        x: null,
        y: null
      },
      items: null
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
    listImages() {
      /*
      let jsonArr = JSON.parse(jsonArrStr);
      let contents = "";
      if (path) { document.getElementById("item-path").innerText = path; }
      for (let i=0; jsonArr.length; i++) {
        if (!jsonArr[i]) {
          break;
        }
        let local_url = "file://" + jsonArr[i].url.replace(/\\/gi, '/');
        let labeled = jsonArr[i].labeled?'highlight':'';
        let labeled_txt = jsonArr[i].labeled?'<img class="ic-checked" src="/img/ic-checked.png">':'';
        contents += '<div class="pure-u-1-12">'
            + '<div class="' + labeled + '" style="background-image: url(\'' + local_url + '\');" onclick="detail_image(\'' + local_url + '\')">'
            + labeled_txt + '</div></div>';
      }
      document.getElementById("id-contents").innerHTML = contents;
      */
    },
    imagePopup(img_url) {
      this.showingPopup = true;
      let img = new Image();
      let local = this;
      img.onload = function(){
        local.canvas.width = img.naturalWidth;
        local.canvas.height = img.naturalHeight;
        local.ctx.drawImage(img, 0, 0, img.naturalWidth, img.naturalHeight);
        // Todo: Rect 정보 fetch 해오기
      };
      img.src = img_url;
    },

    startSelect(e) {
      this.selectionMode = true;
      this.startPosition.x = e.clientX;
      this.startPosition.y = e.clientY;
    },

    drawRect() {
      if (this.selectionMode) {
        // console.log(this.startPosition);
        // this.ctx.beginPath();
        // this.ctx.rect(
        //     this.startPosition.x,
        //     this.startPosition.y,
        //     e.clientX - this.startPosition.x,
        //     e.clientY - this.startPosition.y
        // );
        // this.ctx.closePath();
        // this.ctx.fillRect(0, 0, window.innerWidth, window.innerHeight);
        // this.ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);
        // this.ctx.strokeStyle = "#f00";
        // this.ctx.stroke();
        this.ctx.beginPath();
        this.ctx.lineWidth = "3";
        this.ctx.strokeStyle = "red";
        let imgWidth = this.canvas.width;
        let imgHeight = this.canvas.height;
        let x = imgWidth * 0.136335 - imgWidth * 0.159627 / 2;
        let y = imgHeight * 0.351939 - imgHeight * 0.240893 / 2;
        this.ctx.rect(x, y, imgWidth * 0.159627, imgHeight * 0.240893);
        this.ctx.stroke();
      }
    },
    stopSelect() {
      this.ctx.fillStyle = "#fff";
      this.selectionMode = false;
      this.startPosition.x = null;
      this.startPosition.y = null;
    },
    closePop() {
      this.showingPopup = false;
    }

  },
  mounted() {
    /*
    this.$refs.select.height = window.innerHeight;
    this.$refs.select.width = window.innerWidth;
    */
    this.canvas = this.$refs.canvas;
    this.ctx = this.canvas.getContext("2d");
    this.requestImages();
  }

};
</script>
