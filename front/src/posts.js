function component() {
  var element = document.createElement('div')

  element.innerHTML = 'ねこです！'

  return element
}

document.body.appendChild(component())
