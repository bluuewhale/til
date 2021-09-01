var router = require("express").Router();

const VERSION = "0.0.1";

router.get("/", function (req, res) {
  return res.json({
    version: VERSION,
  });
});

module.exports = router;
