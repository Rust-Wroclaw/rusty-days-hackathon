import "./static/scss/style.scss";

import("./pkg").then((module) => {
  module.run_app();
});
