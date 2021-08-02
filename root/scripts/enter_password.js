const passwordField = document.getElementById("passwordField");
const passwordButton = document.getElementById("enterPassword");

passwordButton.addEventListener('click', function(_e) {console.log("sending pass"); window.ffi.unlock(passwordField.textContent) });