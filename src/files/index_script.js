// main vue entry point
var app = new Vue({
  el: '#app',
  data: {
    content: ''
  },
  methods: {
    changeContent: function(target) {
        let newdata = document.getElementById(target + '-data')
        this.content = newdata.innerHTML
    }
  }
})

// adjust the content window size
// TODO add this to a resize function
var maincontent = document.getElementById('main-content')
maincontent.style.height = window.innerHeight - 10

// set the summary page
app.changeContent('summary')
