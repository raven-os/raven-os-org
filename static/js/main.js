﻿jQuery(document).ready(function ($) {

  if (window.innerWidth < 990) {
    $('.flip-img').addClass('order-2');
    $('.flip-title').addClass('order-1');
  }

  $(window).resize(function () {
    if (window.innerWidth < 990) {
      $('.flip-img').addClass('order-2');
      $('.flip-title').addClass('order-1');
    } else {
      $('.flip-img').removeClass('order-2');
      $('.flip-title').removeClass('order-1');
    }
  });

  function changeNavBar() {
    if ($(this).scrollTop() > 100) {
      $('.back-to-top').fadeIn('slow');
      $('#header').addClass('header-fixed');
    } else {
      $('.back-to-top').fadeOut('slow');
      $('#header').removeClass('header-fixed');
    }
  }

  changeNavBar();
  // Header fixed and Back to top button
  $(window).scroll(changeNavBar);
  $('.back-to-top').click(function () {
    $('html, body').animate({ scrollTop: 0 }, 1500, 'easeInOutExpo');
    return false;
  });

  new Cocoen(document.querySelector('.cocoen'));
  // Initiate the wowjs
  new WOW().init();

  // Initiate superfish on nav menu
  $('.nav-menu').superfish({
    animation: { opacity: 'show' },
    speed: 400
  });

  // Nav Bar
  $("#btn-sub").click(function () {
    $("#nav-menu-container li").removeClass("menu-active");
    $('#li-news').addClass('menu-active');
  });

  // Mobile Navigation
  if ($('#nav-menu-container').length) {
    var $mobile_nav = $('#nav-menu-container').clone().prop({ id: 'mobile-nav' });
    $mobile_nav.find('> ul').attr({ 'class': '', 'id': '' });
    $('body').append($mobile_nav);
    $('body').prepend('<button type="button" id="mobile-nav-toggle"><i class="fa fa-bars"></i></button>');
    $('body').append('<div id="mobile-body-overly"></div>');
    $('#mobile-nav').find('.menu-has-children').prepend('<i class="fa fa-chevron-down"></i>');

    $(document).on('click', '.menu-has-children i', function (e) {
      $(this).next().toggleClass('menu-item-active');
      $(this).nextAll('ul').eq(0).slideToggle();
      $(this).toggleClass("fa-chevron-up fa-chevron-down");
    });

    $(document).on('click', '#mobile-nav-toggle', function (e) {
      $('body').toggleClass('mobile-nav-active');
      $('#mobile-nav-toggle i').toggleClass('fa-times fa-bars');
      $('#mobile-body-overly').toggle();
    });

    $(document).click(function (e) {
      var container = $("#mobile-nav, #mobile-nav-toggle");
      if (!container.is(e.target) && container.has(e.target).length === 0) {
        if ($('body').hasClass('mobile-nav-active')) {
          $('body').removeClass('mobile-nav-active');
          $('#mobile-nav-toggle i').toggleClass('fa-times fa-bars');
          $('#mobile-body-overly').fadeOut();
        }
      }
    });
  } else if ($("#mobile-nav, #mobile-nav-toggle").length) {
    $("#mobile-nav, #mobile-nav-toggle").hide();
  }

  // Smooth scroll on page hash links
  $('a[href*="#"]:not([href="#"])').on('click', function () {
    if (location.pathname.replace(/^\//, '') == this.pathname.replace(/^\//, '') && location.hostname == this.hostname) {

      var target = $(this.hash);
      if (target.length) {
        var top_space = 0;

        if ($('#header').length) {
          top_space = $('#header').outerHeight();

          if (!$('#header').hasClass('header-fixed')) {
            top_space = top_space - 20;
          }
        }

        $('html, body').animate({
          scrollTop: target.offset().top - top_space
        }, 1500, 'easeInOutExpo');

        if ($(this).parents('.nav-menu').length) {
          $('.nav-menu .menu-active').removeClass('menu-active');
          $(this).closest('li').addClass('menu-active');
        }

        if ($('body').hasClass('mobile-nav-active')) {
          $('body').removeClass('mobile-nav-active');
          $('#mobile-nav-toggle i').toggleClass('fa-times fa-bars');
          $('#mobile-body-overly').fadeOut();
        }
        return false;
      }
    }
  });

  // jQuery counterUp
  $('[data-toggle="counter-up"]').counterUp({
    delay: 10,
    time: 1000
  });

  // Red line
  // Get a reference to the <path>
  var path = document.querySelector('#red-line');
  // Get length of path... ~577px in this demo
  var pathLength = path.getTotalLength();
  // Make very long dashes (the length of the path itself)
  path.style.strokeDasharray = pathLength + ' ' + pathLength;

  // Offset the dashes so the it appears hidden entirely
  path.style.strokeDashoffset = pathLength;
  // Current line target percentage
  var lineDrawRatioageTarget = 0;
  // Current line percentage
  var lineDrawRatioage = 0;

  // When the page scrolls...
  window.addEventListener("scroll", function (e) {
    // What % down is it?
    var scrollRatioage = (document.documentElement.scrollTop + document.body.scrollTop) / (document.documentElement.scrollHeight - document.documentElement.clientHeight);

    // Set the target percentage
    lineDrawRatioageTarget = scrollRatioage;
  });
  setInterval(function () {
    // Update line status, proportionally to the distance of the targed
    lineDrawRatioage += (lineDrawRatioageTarget - lineDrawRatioage) * 0.1;
    // Length to offset the dashes
    var drawLength = pathLength * (lineDrawRatioage * 2.1 - 0.15);
    // Draw in reverse
    path.style.strokeDashoffset = pathLength - drawLength;

    // When complete, remove the dash array, otherwise shape isn't quite sharp
    if (lineDrawRatioage >= 0.99) {
      path.style.strokeDasharray = "none";
    } else {
      path.style.strokeDasharray = pathLength + ' ' + pathLength;
    }
  }, 10);
});

function requestBackendNewsletter() {
  var email = document.getElementById("email-subscribe").value;

  if (email === "") {
    return;
  }

  var http = new XMLHttpRequest();
  var url = '/newsletter';
  http.open('POST', url, true);

  //Send the proper header information along with the request
  http.setRequestHeader('Content-type', 'application/json');

  http.onreadystatechange = function () {
    if (http.readyState == 4) {
      var msg = document.getElementById("form-alert");
      msg.style.visibility = 'visible';
      msg.classList.remove("alert-success");
      msg.classList.remove("alert-danger");
      if (http.status == 201) {
        msg.innerHTML = 'You have been sucessfully subscribed!';
        msg.classList.add("alert-success");
        document.getElementById("email-subscribe").value = '';
      } else {
        msg.innerHTML = 'Error: ' + JSON.parse(http.responseText).error_description;
        msg.classList.add("alert-danger");
      }
    }
  }
  var data = JSON.stringify({ "email": email });
  http.send(data);
}
