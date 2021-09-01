var router = require("express").Router();

router.use("/version", require("./versions"));
router.use("/movies", require("./movies"));

module.exports = router;
