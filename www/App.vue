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
    <button style="color:red;" @click="stop()">Stop</button>&nbsp;&nbsp;
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
    <span>
      <div v-for="result in results" :key="result.gen">
        <span v-if="result.done"
          >Answer: <strong>{{ result.best }}</strong
          ><br />With a fitness of <strong>{{ result.fitness }}</strong></span
        >
        <span v-else
          >Best function of Generation {{ result.gen }}
          <strong>({{ result.fitness }})</strong> : {{ result.best }}</span
        >
      </div>
    </span>
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
      running: false,
      results: [],
    }
  },
  mounted: function() {},
  methods: {
    load: function(new_fitness) {
      this.fitness = new_fitness || default_fitness
    },
    start: function() {
      if (this.running) {
        return
      }
      this.results = []
      let fitness_array = this.fitness
        .flat()
        .map((value) => parseInt(value))
        .filter((val) => !isNaN(val))
      if (fitness_array.length === 0 || fitness_array.length % 2 !== 0) {
        console.error('invalid input')
        return
      }
      this.running = true
      let myGP = wasm.GP.new(fitness_array)
      myGP.init()
      const renderLoop = () => {
        if (!this.running) {
          return
        }
        const result = JSON.parse(myGP.tick())
        this.draw_function(result.chromosome)

        this.results.unshift(result)
        if (result.done) {
          this.running = false
          return
        }
        requestAnimationFrame(renderLoop)
      }
      requestAnimationFrame(renderLoop)
    },
    stop: function() {
      this.running = false
    },
    draw_function: function(function_tree) {
      const ctx = document.getElementById('canvas').getContext('2d')
      ctx.clearRect(0, 0, 400, 400)
      ctx.fillStyle = '#000'
      ctx.strokeStyle = '#000'
      //draw origin
      ctx.beginPath()
      ctx.moveTo(0, 200)
      ctx.lineTo(400, 200)
      ctx.moveTo(200, 0)
      ctx.lineTo(200, 400)
      ctx.moveTo(240, 200)
      ctx.lineTo(240, 205)
      ctx.moveTo(280, 200)
      ctx.lineTo(280, 205)
      ctx.moveTo(320, 200)
      ctx.lineTo(320, 205)
      ctx.moveTo(360, 200)
      ctx.lineTo(360, 205)
      ctx.moveTo(400, 200)
      ctx.lineTo(400, 205)

      ctx.moveTo(160, 200)
      ctx.lineTo(160, 205)
      ctx.moveTo(120, 200)
      ctx.lineTo(120, 205)
      ctx.moveTo(80, 200)
      ctx.lineTo(80, 205)
      ctx.moveTo(40, 200)
      ctx.lineTo(40, 205)
      ctx.moveTo(0, 200)
      ctx.lineTo(0, 205)

      ctx.moveTo(200, 240)
      ctx.lineTo(195, 240)
      ctx.moveTo(200, 280)
      ctx.lineTo(195, 280)
      ctx.moveTo(200, 320)
      ctx.lineTo(195, 320)
      ctx.moveTo(200, 360)
      ctx.lineTo(195, 360)
      ctx.moveTo(200, 400)
      ctx.lineTo(195, 400)

      ctx.moveTo(200, 160)
      ctx.lineTo(195, 160)
      ctx.moveTo(200, 120)
      ctx.lineTo(195, 120)
      ctx.moveTo(200, 80)
      ctx.lineTo(195, 80)
      ctx.moveTo(200, 40)
      ctx.lineTo(195, 40)
      ctx.moveTo(200, 0)
      ctx.lineTo(195, 0)
      ctx.stroke()
      ctx.strokeText('x', 390, 210)
      ctx.strokeText('y', 190, 10)
      ctx.strokeText('0', 190, 210)
      ctx.strokeText('1', 235, 220)
      ctx.strokeText('1', 180, 165)
      ctx.strokeText('-1', 155, 220)
      ctx.strokeText('-1', 180, 245)
      ctx.strokeText('2', 275, 220)
      ctx.strokeText('2', 180, 125)
      ctx.strokeText('-2', 115, 220)
      ctx.strokeText('-2', 180, 285)
      ctx.strokeText('3', 315, 220)
      ctx.strokeText('3', 180, 85)
      ctx.strokeText('-3', 75, 220)
      ctx.strokeText('-3', 180, 325)
      ctx.strokeText('4', 355, 220)
      ctx.strokeText('4', 180, 45)
      ctx.strokeText('-4', 35, 220)
      ctx.strokeText('-4', 180, 365)
      ctx.strokeText('5', 390, 220)
      ctx.strokeText('5', 180, 10)
      ctx.strokeText('-5', 5, 220)
      ctx.strokeText('-5', 180, 395)

      //x = 200 + x*40
      //y = 200 - y*40

      for (var i = 0; i < this.fitness.length; i++) {
        ctx.beginPath()
        ctx.arc(
          200 + this.fitness[i][0] * 40,
          200 - this.fitness[i][1] * 40,
          3,
          0,
          2 * Math.PI,
          false
        )
        ctx.fill()
      }

      ctx.beginPath()
      ctx.moveTo(0, this.eval_tree(function_tree, -5.0))
      for (var x = -5.0; x <= 5; x = x + 1 / 40) {
        ctx.lineTo(200 + x * 40, 200 - this.eval_tree(function_tree, x) * 40)
      }
      ctx.strokeStyle = '#0F0'
      ctx.stroke()
    },

    eval_tree: function(tree, x) {
      if (typeof tree == 'undefined') return 0
      if (typeof tree.action == 'number') return tree.action
      if (typeof tree.action == 'string') {
        switch (tree.action) {
          case 'x':
            return x
          case '+':
            return this.eval_tree(tree.arg1, x) + this.eval_tree(tree.arg2, x)
          case '-':
            return this.eval_tree(tree.arg1, x) - this.eval_tree(tree.arg2, x)
          case '*':
            return this.eval_tree(tree.arg1, x) * this.eval_tree(tree.arg2, x)
          case '/':
            return this.eval_tree(tree.arg1, x) / this.eval_tree(tree.arg2, x)
          case 'sin':
            return Math.sin(this.eval_tree(tree.arg1, x))
          case 'cos':
            return Math.cos(this.eval_tree(tree.arg1, x))
          case 'exp':
            return Math.pow(
              this.eval_tree(tree.arg1, x),
              this.eval_tree(tree.arg2, x)
            )
        }
      }
      return 0
    },
  },
}
</script>
