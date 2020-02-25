<template>
  <el-container style="display: flex; flex-direction: row;">
    <el-container style="flex: 1 0 50%;">
      <el-header style="border-bottom: 1px solid #e6e6e6; text-align: left; display: flex; flex-direction: column; padding: 8px; height: auto;">
        <el-input placeholder="Please input" v-model="input1" style="margin-bottom: 8px; flex: 1 1 auto;">
          <template slot="prepend">nats://</template>
        </el-input>
        <el-checkbox-group v-model="msgTypes" style="flex: 1 1 auto;">
          <el-checkbox label="Info"></el-checkbox>
          <el-checkbox label="Ping"></el-checkbox>
          <el-checkbox label="Pong"></el-checkbox>
          <el-checkbox label="OK"></el-checkbox>
          <el-checkbox label="Err"></el-checkbox>
        </el-checkbox-group>
      </el-header>
      <el-main>
        main
      </el-main>
    </el-container>
    <el-aside style="border-left: 1px solid #e6e6e6; flex: 1 0 120px;">
      
    </el-aside>
  </el-container>
</template>

<script>
export default {
  data: () => {
    return {
      msgTypes: []
    }
  }
}
</script>

<style scoped>

</style>