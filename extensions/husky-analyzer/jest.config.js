module.exports = {
    transform: {
        "^.+\\.svelte$": "svelte-jester",
        "^.+\\.js$": "babel-jest",
    },
    moduleFileExtensions: ["js", "svelte"],
    testEnvironment: "jest-environment-jsdom"
};
