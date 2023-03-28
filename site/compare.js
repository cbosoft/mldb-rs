Chart.defaults.datasets.scatter.showLine = true;


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
  // TODO!
  // var e = document.getElementById('details_item');
  // e.style.display = '';
  // e = document.getElementById('details');

  // var experiments_by_group = {};
  // for (exp of cached_data) {
  //   
  // }
}


function plot_losses() {
  var data = { datasets: [] };
  var has_data = false;
  for (exp of cached_data) {
    for ([kind, lossdata] of Object.entries(exp.losses)) {
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

  console.log(data);

  var e = document.getElementById('loss_item');
  e.style.display = has_data ? '' : 'none';
  e = document.getElementById('loss_chart');
  new Chart(e, {
    type: 'scatter',
    data: data,
    options: {
      scales: {
        y: {
          type: 'logarithmic'
        }
      },
      responsive: true,
    }
  });
}


function plot_metrics() {
  const error_chart = document.getElementById('error_chart');
  const error_container = document.getElementById('errors_item');
  plot_metrics_by_pattern(error_chart, error_container, new RegExp('[E]'));

  const correlation_chart = document.getElementById('correlation_chart');
  const correlation_container = document.getElementById('correlations_item');
  plot_metrics_by_pattern(correlation_chart, correlation_container, new RegExp('^[^E]+$'));
}


function plot_metrics_by_pattern(chart_ctx, chart_container, pattern) {
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

  chart_container.style.display = has_data ? '' : 'none';

  new Chart(chart_ctx, {
    type: 'bar',
    data: data,
    options: {
      scales: {
        y: {
          beginAtZero: true
        }
      },
      responsive: true,
    },
  });
}
