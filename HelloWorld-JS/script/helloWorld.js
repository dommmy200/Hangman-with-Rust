function displayHelloWorld() {
    // This is a simple JavaScript function that changes the inner HTML of an element with id 'myDiv' to "Hello, World!".
    const myDiv = document.getElementById('myDiv');
    myDiv.innerHTML = "Hello, World to Webpage!"; // Change the inner HTML of the element.
    myDiv.style.fontFamily = "Arial, sans-serif"; // Change the font family to Arial.
    myDiv.style.color = "blue"; // Change the text color to blue.
    myDiv.style.fontSize = "20px"; // Change the font size to 20px.
    myDiv.style.fontWeight = "bold"; // Make the text bold.
    myDiv.style.textAlign = "center"; // Center the text.
    myDiv.style.marginTop = "20px"; // Add some margin to the top.
    myDiv.style.padding = "10px"; // Add some padding.
    myDiv.style.border = "2px solid black"; // Add a border.
    myDiv.style.borderRadius = "5px"; // Round the corners of the border.
    myDiv.style.backgroundColor = "lightyellow"; // Change the background color to light yellow.
    myDiv.style.boxShadow = "2px 2px 5px gray"; // Add a shadow effect.
    myDiv.style.transition = "all 0.3s"; // Add a transition effect.
    myDiv.onmouseover = function() { // Add a mouseover effect.
        myDiv.style.backgroundColor = "lightblue"; // Change the background color to light blue on hover.
    };
    myDiv.onmouseout = function() { // Add a mouseout effect.
        myDiv.style.backgroundColor = "lightyellow"; // Change the background color back to light yellow on mouse out.
    };
    // This is a simple JavaScript function that logs "Hello, World!" to the console.
    console.log("Hello, World to Console!");
}
displayHelloWorld();
