document.addEventListener("DOMContentLoaded", () => {
    const exercise = new URL(document.location).searchParams.get("ex");
    const tabSelector = document.querySelector(`#ex${exercise}`);
    if (tabSelector) {
        tabSelector.checked = true;
    }

    const navItems = Array.prototype.slice.call(document.querySelectorAll(".navbar-item"), 0);
    for (let navItem of navItems) {
        navItem.addEventListener("click", (event) => {
            const exercise = event.currentTarget.getAttribute("for").match(/\d+/)[0];
            const url = new URL(window.location);
            url.searchParams.set("ex", exercise);
            history.pushState(null, "", url);
        }); 
    }

     const navbarBurger = document.querySelector(".navbar-burger");
     navbarBurger.addEventListener("click", () => {
        const targetId = navbarBurger.dataset.target;
        const target = document.getElementById(targetId);
        navbarBurger.classList.toggle("is-active");
        target?.classList.toggle("is-active");
    });
});
