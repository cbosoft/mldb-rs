Chart.defaults.datasets.scatter.showLine = true;
var canvases = {};


function init() {
  const urlParams = new URLSearchParams(window.location.search);
  const groups = urlParams.getAll('group');
  const expids = urlParams.getAll('expid');
  if (groups.length) {
    get_groups(groups);
  }
  else {
    get_experiments(expids);
  }
}


function get_groups(groups) {
  send_data({ command: "get_groups", groups: groups })
    .then(response => response.json())
    .then(show_data);
}


function get_experiments(expids) {
  send_data({ command: "get_experiments", expids: expids })
    .then(response => response.json())
    .then(show_data);
}


var cached_data = {};

function show_data(data) {
  cached_data = data;
  init_settings();
  reshow_data();
  refresh_in_a_bit();
}


function init_settings() {
  // TODO!
}


function reshow_data() {
  display_details();
  plot_losses();
  plot_metrics();
}


function display_details() {
  var e = document.getElementById('details_item');
  if (cached_data.length == 1) {
    var exp = cached_data[0];

    e.style.display = '';
    e = document.getElementById('details');

    document.getElementById('details-expid').innerHTML = exp.expid;
    document.getElementById('details-status').innerHTML = exp.status;
    if (exp.losses) {
      document.getElementById('details-epochs').innerHTML = exp.losses.valid[exp.losses.valid.length - 1][0];
    }
    else {
      document.getElementById('details-epochs').innerHTML = '';
    }

    if (exp.groups) {
      if (exp.groups.length > 1) {
        document.getElementById('details-groups').innerHTML = "<ul><li>" + exp.groups.join("</li><li>") + "</li></ul>";
      }
      else {
        document.getElementById('details-groups').innerHTML = exp.groups[0].replace(/;/g, "; <br/>");
      }
    }
    else {
      document.getElementById('details-groups').innerHTML = '';
    }
  }
  else {
    e.style.display = 'none';
  }
}


function plot_losses() {
  var data = { datasets: [] };
  var has_data = false;
  for (exp of cached_data) {
    kinds = Object.keys(exp.losses);
    kinds.sort();
    for (kind of kinds) {
      var lossdata = exp.losses[kind];
      var dataset = {
        data: [],
        label: exp.expid + "/" + kind
      };

      for ([e, v] of lossdata) {
        dataset.data.push({ x: e, y: v });
        has_data = true;
      }
      data.datasets.push(dataset);
    }
  }

  var e = document.getElementById('loss_item');
  e.style.display = has_data ? '' : 'none';
  e = document.getElementById('loss_chart');

  if (!canvases.hasOwnProperty('loss_chart')) {
    canvases['loss_chart'] = new Chart(e, {
      type: 'scatter',
      data: [],
      options: {
        scales: {
          y: {
            type: 'logarithmic'
          }
        },
        responsive: true,
        animation: false,
      }
    });
  }

  canvases['loss_chart'].data = data;
  canvases['loss_chart'].update();
}


function plot_metrics() {
  plot_metrics_by_pattern('error_chart', 'errors_item', new RegExp('[E]'));
  plot_metrics_by_pattern('correlation_chart', 'correlations_item', new RegExp('^[^E]+$'));
}


function plot_metrics_by_pattern(chart_id, chart_container_id, pattern) {
  var chart_ctx = document.getElementById(chart_id);
  var data = { labels: [], datasets: [] };
  var has_data = false;
  for (i in cached_data) {
    var exp = cached_data[i];
    var dataset = {
      data: [],
      label: exp.expid,
    };
    metrics = Object.keys(exp.metrics);
    metrics.sort();
    for (mname of metrics) {
      if (pattern.test(mname)) {
        mvalue = exp.metrics[mname];
        if (!data.labels.includes(mname)) {
          data.labels.push(mname);
        }
        if (mvalue < 0.0) {
          mvalue = NaN;
        }
        dataset.data.push(mvalue);
        has_data = true;
      }
    }
    data.datasets.push(dataset);
  }

  var chart_container = document.getElementById(chart_container_id);
  chart_container.style.display = has_data ? '' : 'none';

  if (!canvases.hasOwnProperty(chart_id)) {
    canvases[chart_id] = new Chart(chart_ctx, {
      type: 'bar',
      data: [],
      options: {
        scales: {
          y: {
            beginAtZero: true
          }
        },
        responsive: true,
        animation: false,
      },
    });
  }

  canvases[chart_id].data = data;
  canvases[chart_id].update();
}

function refresh_in_a_bit() {
  setTimeout(init, 5000);
}
