<template>
  <el-container class="body">
    <el-header class="header">
      <span class="header-title"> SimpleGain </span>
    </el-header>

    <el-card class="box-card">
      <!-- <div class="block"> -->
      <el-slider
        v-model="gain"
        vertical
        :max="1"
        :min="0"
        :step="0.01"
        height="340px"
        ref="gain"
        class="gain-slider"
      >
      </el-slider>
      <!-- </div> -->
    </el-card>
  </el-container>
</template>

<script>
export default {
  components: {},
  data() {
    return {
      gain: 0,
    };
  },
  mounted() {
    this.update();
    const gain = this.$refs.gain;
    gain.addEventListener("input", (e) => {
      this.gain = e.target.value;
    });
    gain.addEventListener("onmousedown", (e) => {
      external.invoke("mouseOverGain");
    });
    gain.addEventListener("onmouseup", (e) => {
      external.invoke("releaseGain");
    });
    document.oncontextmenu = function () {
      return false;
    };
    document.addEventListener(
      "touchmove",
      function (e) {
        e.preventDefault();
      },
      { passive: false }
    );
  },
  methods: {
    getter: function () {
      this.gain = Number(external.invoke("getGain"));
    },
    update: function () {
      const self = this;
      this.interval = setInterval(function () {
        self.getter();
      }, 1);
    },
  },
  watch: {
    gain: function () {
      external.invoke("setGain " + this.gain);
    },
  },
};
</script>

<style>
.body {
  width: 480px;
  height: 480px;
  background-color: #ffffff;
}

.box-card {
  width: 440px;
  height: 400px;
  margin-left: 18px;
  margin-right: 18px;
  background-color: #ffffff;
}

.gain-slider {
  padding: 0px 0px 0px 180px;
}

.header-title {
  vertical-align: middle;
  display: inline-block;
  font-size: 32px;
  /* padding: 0px 0px 10px 10px; */
  color: #131212;
  font-family: "Baloo Tammudu 2", cursive;
}
</style>