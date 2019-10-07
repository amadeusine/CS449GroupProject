var express = require('express');
var router = express.Router();

/* GET home page. */
router.get('/', function(req, res, next) {
  
  let options = {
    title: 'Nine Men\'s Morris',
    theme: req.query.theme || 'default',
  }
  res.render('index', options);
});

module.exports = router;
