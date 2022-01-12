<template>
  <div class="plugin-config">
    <div>
      <input
        type="checkbox"
        id="crop"
        v-model="crop"
        v-on:change="updateConfig()"
      />
      <label for="crop">Crop first detection</label>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
export default defineComponent({
  name: "Widget",
  props: {
    defaultConfig: Object,
  },
  data() {
    return {
      crop: false,
    };
  },
  methods: {
    async updateConfig() {
      let config = {
        crop: this.crop,
      };
      this.$emit("changeConfig", config);
    },
  },
  created() {
    this.crop = this.defaultConfig.crop;
    this.updateConfig();
  },
});
</script>

<style scoped lang="css">
.plugin-config {
  margin: 5px;
  min-width: 100px;
  min-height: 50px;
}
</style>
