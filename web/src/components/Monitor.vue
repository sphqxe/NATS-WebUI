<template>
  <el-container style="display: flex; flex-direction: row;">
    <div id="subjects" style="flex: 1 1 60%">
      Subjects
    </div>
    <div id="details" style="flex: 1 1 40%">
      {{ server }}
    </div>
  </el-container>
</template>

<script>
import { mapState } from 'vuex'

export default {
  computed: {
    ...mapState({
      server: s => s.transient.serversMap[s.transient.selectedServer]
    })
  }
}
</script>

<style scoped>

</style>