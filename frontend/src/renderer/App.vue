<template>
  <div id="app">
  	<router-view :key="$route.fullPath"></router-view>
  </div>

</template>

<script>
  export default {
    name: 'match'
  }
</script>

<style>
 #app
 {
 	height:800px;
 	width:100%;
 	min-height:100%;

 }
 body
 {
 	margin:0px;
 }
</style>
