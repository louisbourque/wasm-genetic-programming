<template>
  <div>
    <button @click="load()">Clear</button>&nbsp;&nbsp;
    <button @click="load(a_fitness)">
      Load: 3x<sup>3</sup> + 2x<sup>2</sup> + x + 1</button
    >&nbsp;&nbsp;
    <button @click="load(b_fitness)">Load: cos(x) + 3sin(x<sup>2</sup>)</button
    >&nbsp;&nbsp;
    <button @click="load(c_fitness)">Load: x + 1</button>&nbsp;&nbsp;
    <button @click="start()">Start</button>&nbsp;&nbsp;
    <br />
    <canvas
      id="canvas"
      width="400"
      height="400"
      style="border: 1px solid black; float: left"
    ></canvas>
    <br />
    <div id="inputs">
      <table>
        <thead>
          <th>x</th>
          <th>f(x)</th>
        </thead>
        <tbody>
          <tr v-for="(value, index) in fitness" :key="index">
            <td><input type="text" v-model="value[0]" /></td>
            <td><input type="text" v-model="value[1]" /></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<script>
import * as wasm from 'wasm-genetic-programming'

const a_fitness = [
  [-5, -329],
  [-4.5, -236.375],
  [-4, -163],
  [-3.5, -106.625],
  [-3, -65],
  [-2.5, -35.875],
  [-2, -17],
  [-1.5, -6.125],
  [-1, -1],
  [-0.5, 0.625],
  [0, 1],
  [0.5, 2.375],
  [1, 7],
  [1.5, 17.125],
  [2, 35],
  [2.5, 62.875],
  [3, 103],
  [3.5, 157.625],
  [4, 229],
  [4.5, 319.375],
  [5, 431],
]
const b_fitness = [
  [-5, -0.11339306483009282],
  [-4.5, 2.745779535264579],
  [-4, -1.5173535708588077],
  [-3.5, -1.8698147522341784],
  [-3, 0.24636295912482442],
  [-2.5, -0.9006812651896041],
  [-2, -2.686554322470927],
  [-1.5, 2.404956792331467],
  [-1, 3.064715260291829],
  [-0.5, 1.6197944396539414],
  [0, 1],
  [0.5, 1.6197944396539414],
  [1, 3.064715260291829],
  [1.5, 2.404956792331467],
  [2, -2.686554322470927],
  [2.5, -0.9006812651896041],
  [3, 0.24636295912482442],
  [3.5, -1.8698147522341784],
  [4, -1.5173535708588077],
  [4.5, 2.745779535264579],
  [5, -0.11339306483009282],
]
const c_fitness = [
  [-5, -4],
  [-4.5, -3.5],
  [-4, -3],
  [-3.5, -2.5],
  [-3, -2],
  [-2.5, -1.5],
  [-2, -1],
  [-1.5, -0.5],
  [-1, 0],
  [-0.5, 0.5],
  [0, 1],
  [0.5, 1.5],
  [1, 2],
  [1.5, 2.5],
  [2, 3],
  [2.5, 3.5],
  [3, 4],
  [3.5, 4.5],
  [4, 5],
  [4.5, 5.5],
  [5, 6],
]

const default_fitness = [
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
  ['', ''],
]

export default {
  name: 'App',

  data: () => {
    return {
      fitness: default_fitness,
      a_fitness,
      b_fitness,
      c_fitness,
      default_fitness,
    }
  },
  mounted: function () {
    console.log(this)
  },
  methods: {
    load: function (new_fitness) {
      this.fitness = new_fitness || default_fitness
    },
    start: function () {
      let fitness_array = this.fitness
        .flat()
        .map((value) => parseInt(value))
        .filter((val) => !isNaN(val))
      if (fitness_array.length === 0 || fitness_array.length % 2 !== 0) {
        console.error('invalid input')
        return
      }
      let myGP = wasm.GP.new(fitness_array)
      myGP.run()
    },
  },
}
</script>
