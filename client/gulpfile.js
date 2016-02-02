var gulp = require('gulp');
var $ = require('gulp-load-plugins')();
var argv = require('yargs').argv;
var rimraf = require('rimraf');
var sequence = require('run-sequence');

var isProduction = !!(argv.production);

var paths = {
	scss: 'scss/main.scss',
	scss_inc: [
		'scss/main.scss',
		'node_modules/foundation-sites/scss/',
		'node_modules/motion-ui/src/'
	],
	app_js: [
		'js/main.js',
	],
	foundation_js: [
		'node_modules/foundation-sites/js/foundation.core.js',
		'node_modules/foundation-sites/js/foundation.abide.js',
		'node_modules/foundation-sites/js/foundation.accordion.js',
		'node_modules/foundation-sites/js/foundation.accordionMenu.js',
		'node_modules/foundation-sites/js/foundation.drilldown.js',
		'node_modules/foundation-sites/js/foundation.dropdown.js',
		'node_modules/foundation-sites/js/foundation.dropdownMenu.js',
		'node_modules/foundation-sites/js/foundation.equalizer.js',
		'node_modules/foundation-sites/js/foundation.interchange.js',
		'node_modules/foundation-sites/js/foundation.magellan.js',
		'node_modules/foundation-sites/js/foundation.offcanvas.js',
		'node_modules/foundation-sites/js/foundation.orbit.js',
		'node_modules/foundation-sites/js/foundation.responsiveMenu.js',
		'node_modules/foundation-sites/js/foundation.responsiveToggle.js',
		'node_modules/foundation-sites/js/foundation.reveal.js',
		'node_modules/foundation-sites/js/foundation.slider.js',
		'node_modules/foundation-sites/js/foundation.sticky.js',
		'node_modules/foundation-sites/js/foundation.tabs.js',
		'node_modules/foundation-sites/js/foundation.toggler.js',
		'node_modules/foundation-sites/js/foundation.tooltip.js',
		'node_modules/foundation-sites/js/foundation.util.box.js',
		'node_modules/foundation-sites/js/foundation.util.keyboard.js',
		'node_modules/foundation-sites/js/foundation.util.mediaQuery.js',
		'node_modules/foundation-sites/js/foundation.util.motion.js',
		'node_modules/foundation-sites/js/foundation.util.nest.js',
		'node_modules/foundation-sites/js/foundation.util.timerAndImageLoader.js',
		'node_modules/foundation-sites/js/foundation.util.touch.js',
		'node_modules/foundation-sites/js/foundation.util.triggers.js'
	],
	angular_js: [
		'node_modules/angular/angular.js',
		'node_modules/fastclick/lib/fastclick.js',
		'node_modules/angular-ui-router/release/angular-ui-router.js',
	],
	html: [
		'*.html'
	],
	templates: [
		'templates/*.html'
	],
	jquery: [
		'node_modules/jquery/dist/jquery.min.js'
	],
};

gulp.task('clean', function(cb) {
	rimraf('./build', cb);
});

gulp.task('copy', function() {
	sequence(['copy:html', 'copy:jquery'], 'copy:templates');
});

gulp.task('copy:html', function() {
	return gulp.src(paths.html)
		.pipe(gulp.dest('./build'));
});

gulp.task('copy:templates', function() {
	return gulp.src(paths.templates, { base: 'templates/' })
		.pipe(gulp.dest('./build/templates/'));
});

gulp.task('copy:jquery', function() {
	return gulp.src(paths.jquery, { base: 'node_modules/jquery/dist/' })
		.pipe(gulp.dest('./build/js'));
});

gulp.task('sass', function() {
	return gulp.src(paths.scss)
		.pipe($.sass({includePaths: paths.scss_inc,
			outputStyle: (isProduction ? 'compressed' : 'nested'),
			errLogToConsole: true
		}))
		.pipe($.cssnano())
		.pipe(gulp.dest('./build/css/'));
});

gulp.task('uglify', ['uglify:angular', 'uglify:foundation', 'uglify:app']);

gulp.task('uglify:foundation', function() {
	return gulp.src(paths.foundation_js)
		.pipe($.uglify())
		.pipe($.concat('foundation.min.js'))
		.pipe(gulp.dest('./build/js'));
});

gulp.task('uglify:angular', function() {
	return gulp.src(paths.angular_js)
		.pipe($.uglify())
		.pipe($.concat('angular.min.js'))
		.pipe(gulp.dest('./build/js'));
});

gulp.task('uglify:app', function() {
	var uglify = $.if(isProduction, $.uglify().on('error', function (e) { console.log(e); }));

	return gulp.src(paths.app_js)
		.pipe(uglify)
		.pipe($.concat('app.min.js'))
		.pipe(gulp.dest('./build/js'));
});

gulp.task('watch', function() {
	gulp.watch(paths.app_js, ['uglify:app']);
	gulp.watch(paths.scss, ['sass']);
	gulp.watch(paths.html, ['copy:html']);
	gulp.watch(paths.templates, ['copy:templates']);
});

gulp.task('build', function(cb) {
	sequence('clean', ['copy', 'sass', 'uglify'], cb);
});

gulp.task('server', ['build'], function() {
  gulp.src('./build')
    .pipe($.webserver({
      port: 8079,
      host: 'localhost',
      fallback: 'index.html',
      livereload: true,
      open: true
    }))
  ;
});

gulp.task('default', ['build', 'watch']);
