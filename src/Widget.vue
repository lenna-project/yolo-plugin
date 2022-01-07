<template>
  <div class="plugin-config"></div>
</template>

<script>
import { defineComponent } from "vue";
export default defineComponent({
  name: "Widget",
  props: {
    defaultConfig: Object,
  },
  data() {
    return {};
  },
  methods: {
    async updateConfig() {
      let config = {};
      this.$emit("changeConfig", config);
    },
  },
  created() {
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
