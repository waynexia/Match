<template>
  <div id="frame">
  	<sidebar></sidebar>
    <router-view></router-view>
  </div>

</template>

<script>
  import sidebar from './components/Sidebar'
  export default {
    name: 'match',
    components:{
    	sidebar
    }
  }
</script>

<style>
 #frame
 {
 	height:800px;
 	width:100%;
  min-height: 100%;
 }
</style>
