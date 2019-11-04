const express = require('express');
const router = express.Router();

/* GET logic listing. */
router.get('/logic', function(req, res, next) {
  res.send(JSON.stringify({a:2}));
});

module.exports = router;