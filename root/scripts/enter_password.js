const passwordField = document.getElementById("passwordField");
const passwordButton = document.getElementById("enterPassword");

passwordButton.addEventListener('click', function(_e) { window.ffi.unlock(passwordField.textContent) });