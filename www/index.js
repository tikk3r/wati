// Code examples used:
// https://github.com/travisdoesmath/n-pendulum-wasm/
// https://d3fc.io/examples/chart-d3fc-zoom/
// https://blog.scottlogic.com/2020/05/01/rendering-one-million-points-with-d3.html
//
// If you only use `npm` you can simply
// import { Chart } from "wasm-demo" and remove `setup` call from `bootstrap.js`.
class Chart {}
// OLD: import { shared_memory } from "wasm-demo";
// shared_memory no longer needed — plot_interferometer_uvcoverage returns Float64Array directly

const array_selector = document.getElementById("array_selector");
const band_selector = document.getElementById("band_selector");

const phi = document.getElementById("phi");
const duration = document.getElementById("duration");
const frequency = document.getElementById("frequency");
const frequency_channels = document.getElementById("frequency_channels");
const declination = document.getElementById("declination");
const colour_points = document.getElementById("checkbox_colour_by_freq");
const reset = document.getElementById("btn_reset_sim");
const btn_save_img = document.getElementById("btn_save_img");

const control_uvcov = document.getElementById("uvcov-control");
const status = document.getElementById("status");
const antennas = document.getElementById("antennas");

var antenna_list = {"LOFAR": ['CS001HBA0', 'CS001HBA1', 'CS002HBA0', 'CS002HBA1', 'CS003HBA0', 'CS003HBA1', 'CS004HBA0', 'CS004HBA1', 'CS005HBA0', 'CS005HBA1', 'CS006HBA0', 'CS006HBA1', 'CS007HBA0', 'CS007HBA1', 'CS011HBA0', 'CS011HBA1', 'CS013HBA0', 'CS013HBA1', 'CS017HBA0', 'CS017HBA1', 'CS021HBA0', 'CS021HBA1', 'CS024HBA0', 'CS024HBA1', 'CS028HBA0', 'CS028HBA1', 'CS030HBA0', 'CS030HBA1', 'CS031HBA0', 'CS031HBA1', 'CS032HBA0', 'CS032HBA1', 'CS101HBA0', 'CS101HBA1', 'CS103HBA0', 'CS103HBA1', 'CS201HBA0', 'CS201HBA1', 'CS301HBA0', 'CS301HBA1', 'CS302HBA0', 'CS302HBA1', 'CS401HBA0', 'CS401HBA1', 'CS501HBA0', 'CS501HBA1', 'RS106HBA', 'RS205HBA', 'RS208HBA', 'RS210HBA', 'RS305HBA', 'RS306HBA', 'RS307HBA', 'RS310HBA', 'RS406HBA', 'RS407HBA', 'RS409HBA', 'RS503HBA', 'RS508HBA', 'RS509HBA', 'DE601HBA', 'DE602HBA', 'DE603HBA', 'DE604HBA', 'DE605HBA', 'FR606HBA', 'SE607HBA', 'UK608HBA', 'DE609HBA', 'PL610HBA', 'PL611HBA', 'PL612HBA', 'IE613HBA', 'LV614HBA', "BG", "IT", "GMRT", "IT-NOTO", "CZ", "CZ-Ondrejov", "Gap Filler"].sort(),
    "e-MERLIN": ["Lovell", "MarkII", "Defford", "Knockin", "Pickmere", "Darnhall", "Cambridge"].sort(),
};

/** Add event listeners. */
function setupUI() {
    status.innerText = "WebAssembly loaded!";
	phi.addEventListener("change", updatePlot);
	phi.addEventListener("input", updatePlot);
	duration.addEventListener("change", updatePlot);
	duration.addEventListener("input", updatePlot);
	frequency.addEventListener("change", updatePlot);
	frequency.addEventListener("input", updatePlot);
	frequency_channels.addEventListener("change", updatePlot);
	frequency_channels.addEventListener("input", updatePlot);
	time_channels.addEventListener("change", updatePlot);
	time_channels.addEventListener("input", updatePlot);
	declination.addEventListener("change", updatePlot);
	declination.addEventListener("input", updatePlot);
	array_selector.addEventListener("input", updateAntennas);
	array_selector.addEventListener("change", updateAntennas);

	band_selector.addEventListener("input", updateFrequencies);
	band_selector.addEventListener("change", updateFrequencies);

    colour_points.addEventListener("change", updatePlot);
    reset.addEventListener("click", resetSliders);

    updateAntennas();
}

function resetSliders() {
    console.log("Resetting");
    declination.value = 58;
    duration.value = 480;
    phi.value = 120;
    time_channels.value = 3;
    frequency.value = 144;
    frequency_channels.value = 1;
    updatePlot();
}

var data = []

const x = d3.scaleLinear().domain([-1250e3, 1250e3]);
const y = d3.scaleLinear().domain([-1250e3, 1250e3]);

const pointSeries = fc
    .seriesWebglPoint( )
    .equals((previousData, currentData) => previousData === currentData)
    .crossValue(d => d.x)
    .mainValue(d => d.y)
    .size(8);

// create a d3fc-zoom that handles the mouse / touch interactions
const zoom = fc.zoom().on('zoom', render);

const gridline = fc.annotationCanvasGridline().xTicks(40).yTicks(40);


const axis = fc
    .axisBottom(x)
    .decorate(sel => {
        sel.enter()
            .append('text')
    .attr('fill', 'red');
    });

// the chart!
const chart = fc
    .chartCartesian(x, y)
    .canvasPlotArea(gridline)
    .webglPlotArea(pointSeries)
    .xLabel("u [λ]")
    .yLabel("v [λ]")
    .decorate(sel => {
        // add the zoom interaction on the enter selection
        // use selectAll to avoid interfering with the existing data joins
        sel.enter()
            .selectAll('.plot-area')
            .call(zoom, x, y);
        sel.enter()
            .selectAll('.x-axis')
            .call(zoom, x, null);
        sel.enter()
            .selectAll('.y-axis')
            .call(zoom, null, y);
    })
    .xDecorate( sel => {
        sel.select('text')
        .attr('transform', 'rotate(-25) translate(0 25)')
        .style('font-size', '16px')
        .style('font-family', 'Spectral');
    })
    .yDecorate( sel => {
        sel.select('text')
        //.attr('transform', 'rotate(-45 35 15)')
        .style('font-size', '16px')
        .style('font-family', 'Spectral')
    });

const webglColor = color => {
  const { r, g, b, opacity } = d3.color(color).rgb();
  return [r / 255, g / 255, b / 255, opacity];
};

function render() {
    // Set new data on your chart:
    //var items = d3.select('#chart').selectAll('*').remove();
    let cv = document.getElementById("chart");

    d3.select('#chart')
        .style("font-size", "32px")
        .style("font-family", "Spectral")
        .datum(data)
        .call(chart);
}

/** Main entry point */
export function main() {
    setupUI();
    resetSliders();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart) {
    Chart = WasmChart;
}

function toggleButtonsCore() {
    const array = document.querySelector("input[name=array]:checked");
    let telescope = document.querySelector(`label[for=${array.id}]`).innerHTML;

    for (var i = 0; i < antenna_list[telescope].length; i++) {
        let ant = antenna_list[telescope][i];
        if (ant.includes("CS")) {
            let checkbox = document.getElementById(ant);
            checkbox.checked = !checkbox.checked;
        }
    }
    updatePlot();
}

function toggleButtonsRemote() {
    const array = document.querySelector("input[name=array]:checked");
    let telescope = document.querySelector(`label[for=${array.id}]`).innerHTML;

    for (var i = 0; i < antenna_list[telescope].length; i++) {
        let ant = antenna_list[telescope][i];
        if (ant.includes("RS")) {
            let checkbox = document.getElementById(ant);
            checkbox.checked = !checkbox.checked;
        }
    }
    updatePlot();
}

function toggleButtonsIntl() {
    const array = document.querySelector("input[name=array]:checked");
    let telescope = document.querySelector(`label[for=${array.id}]`).innerHTML;

    for (var i = 0; i < antenna_list[telescope].length; i++) {
        let ant = antenna_list[telescope][i];
        if (!ant.includes("CS") && !ant.includes("RS") && !ant.includes("BG") && !ant.includes("IT") && !ant.includes("GMRT") && !ant.includes("CZ")) {
            let checkbox = document.getElementById(ant);
            checkbox.checked = !checkbox.checked;
        }
    }
    updatePlot();
}

function updateFrequencies() {
    const selected_band = document.querySelector("input[name=band]:checked");
    const band = selected_band.id;
    //const band = document.querySelector(`label[for=${selected_band.id}]`).innerHTML;

    console.log(band);
    if (band == "HBA") {
        frequency.max = 168;
        frequency.min = 120;
        frequency.value = 144;
    }else if (band == "LBA") {
        frequency.max = 90;
        frequency.min = 10;
        frequency.value = 58;
    }

    if (band == "L") {
        frequency.max = 1740;
        frequency.min = 1230;
        frequency.value = 1500;
    }else if (band == "C") {
        frequency.max = 7500;
        frequency.min = 4300;
        frequency.value = 5000;
    }else if (band == "K") {
        frequency.max = 25000;
        frequency.min = 19000;
        frequency.value = 22000;
    }
    document.getElementById("label_freq").innerText = `Observing frequency: ${frequency.value} MHz`;
    updatePlot();
}

function updateFrequencyBands(telescope) {
    var bands = [];
    if (telescope == "LOFAR") {
        bands = ["LBA", "HBA"];
    } else if (telescope == "e-MERLIN") {
        bands = ["L", "C", "K"];
    }

    let freqfield = document.getElementById("freqbands");
    freqfield.innerHTML = '';
    let leg = document.createElement("legend");
    leg.innerHTML = "Observing band";
    freqfield.appendChild(leg);

    let cblist = document.createElement("ul");
    cblist.id = "radiolist";

    var first = true;
    bands.forEach((b) => {
        let li = document.createElement("li");

        let checkbox = document.createElement("input");
        checkbox.type = "radio";
        checkbox.name = "band";
        checkbox.id = b;
        if (telescope == "LOFAR" && b == "HBA") {
            checkbox.checked = true;
            first = false;
        }else if (telescope == "e-MERLIN" && b == "L") {
            checkbox.checked = true;
            first = false;
        }
        checkbox.addEventListener('change', updateFrequencies);

        let label= document.createElement("label");
        label.for = b;
        label.appendChild(checkbox);

        let description = document.createTextNode(b);
        label.appendChild(description);

        li.appendChild(label);
        cblist.appendChild(li);
    });
    freqfield.appendChild(cblist);
    updateFrequencies();
}

function updateAntennas() {
    const array = document.querySelector("input[name=array]:checked");
    let telescope = document.querySelector(`label[for=${array.id}]`).innerHTML;

    antennas.innerHTML = '';
    let leg = document.createElement("legend");
    leg.innerHTML = "Antennas";
    antennas.appendChild(leg);
    
    if (telescope == "LOFAR") {
        let btn_core = document.createElement("input");
        btn_core.type = "button";
        btn_core.id = "btn_lofar_core";
        btn_core.value = "Toggle\nCS"
        btn_core.classList.add("button");
        btn_core.addEventListener('click', toggleButtonsCore);

        let btn_remote = document.createElement("input");
        btn_remote.type = "button";
        btn_remote.id = "btn_lofar_remote";
        btn_remote.value = "Toggle\nRS"
        btn_remote.classList.add("button");
        btn_remote.addEventListener('click', toggleButtonsRemote);

        let btn_intl = document.createElement("input");
        btn_intl.type = "button";
        btn_intl.id = "btn_lofar_intl";
        btn_intl.value = "Toggle\nIntl."
        btn_intl.classList.add("button");
        btn_intl.addEventListener('click', toggleButtonsIntl);

        antennas.appendChild(btn_core);
        antennas.appendChild(btn_remote);
        antennas.appendChild(btn_intl);
        antennas.appendChild(document.createElement("br"));
        antennas.appendChild(document.createElement("br"));
    }

    let cblist = document.createElement("ul");
    cblist.id = "checkboxlist";
    for (var i = 0; i < antenna_list[telescope].length; i++) {
        let ant = antenna_list[telescope][i];

        let li = document.createElement("li");

        let checkbox = document.createElement("input");
        checkbox.type = "checkbox";
        checkbox.name = ant;
        checkbox.id = ant;
        checkbox.value = `include_${ant}`;
        if (ant.includes("CS") || ant.includes("BG") || ant.includes("IT") || ant.includes("GMRT") || ant.includes("CZ")) {
            checkbox.checked = false;
        } else {
            checkbox.checked = true;
        }
        checkbox.addEventListener('change', updatePlot);

        let label= document.createElement("label");
        label.appendChild(checkbox);

        let description = document.createTextNode(ant);
        label.appendChild(description);

        li.appendChild(label);
        cblist.appendChild(li);
    }
    antennas.appendChild(cblist);
    updateFrequencyBands(telescope);
    updatePlot();
}

function updatePlotUvCoverage() {
	let phi_value = Number(phi.value) / 60.0 - 6;
	let duration_value = Number(duration.value) / 60.0;
	let t_channels = time_channels.value;
    const TSLOT_LIMIT = 20;
    if (t_channels > duration_value * TSLOT_LIMIT) {
        t_channels = Math.ceil(duration_value * TSLOT_LIMIT);
    }
	let freq_value = Number(frequency.value) * 1e6;
	let freq_channels = frequency_channels.value;
    let dec_value = Number(declination.value) * Math.PI / 180.0;

    const array = document.querySelector("input[name=array]:checked");
    let telescope = document.querySelector(`label[for=${array.id}]`).innerHTML;

    document.getElementById("label_dec").innerText = `Declination w.r.t. celestial equator: ${(dec_value * 180.0/Math.PI).toFixed(2)} deg`;
    document.getElementById("label_time").innerText = `Offset from noon: ${(phi_value).toFixed(2)} h`;
    document.getElementById("label_itime").innerText = `Integration time: ${duration_value.toFixed(2)} h`;
    document.getElementById("label_ntimes").innerText = `Time samples: ${t_channels}`;
    document.getElementById("label_bandwidth").innerText = `Frequency channels: ${freq_channels}`;
    document.getElementById("label_freq").innerText = `Observing frequency: ${freq_value / 1e6} MHz`;

    let antmask = new Uint8Array(antenna_list[telescope].length);
    for (var i = 0; i < antenna_list[telescope].length; i++) {
        let ant = antenna_list[telescope][i];
        let cb = document.getElementById(ant);
        antmask[i] = cb.checked ? 1 : 0;
    }

	// OLD (use-after-free): returned dangling pointer from Rust, manually constructed Float64Array view
    // let uvptr = Chart.plot_interferometer_uvcoverage(dec_value, freq_value, freq_channels, phi_value, duration_value, t_channels, telescope, antmask);
    // var Nant = 0;
    // if (telescope == "LOFAR") {
    //     Nant = 71;
    // } else if (telescope == "e-MERLIN") {
    //     Nant = 7;
    // }
    // Nant = antmask.reduce((a, b) => a + b, 0);
    // let Nbaselines = Nant * (Nant - 1) / 2;
    // let Nvalues = ((Nant + Nbaselines)) * freq_channels * t_channels * 2;
    // const memory = shared_memory();
    // let uv_points = new Float64Array(memory.buffer, uvptr, Nvalues);

    let uv_points = Chart.plot_interferometer_uvcoverage(dec_value, freq_value, freq_channels, phi_value, duration_value, t_channels, telescope, antmask);

    let arr = Array.from(uv_points);
    let full_uv = arr.flatMap((coord) => [coord, -coord]);
    let freqs = [];
    let a = 0;
    for(a; a<=freq_channels; a++){
        freqs.push(freq_value + a * 5e6);
    }
    data = [];
    let freq_idx = -1;

    /*
    // Iterate per 2 because we have u, -u, v, -v in the array.
    // For every time slot
    // Nbaselines + Nant antenna points
    // freq_channels frequency points
    let t = -1;
    d3.range(0, uv_points.length+2, 2).forEach(d => {
        let trem = d % (2 * (((Nbaselines + Nant) * freq_channels)));
        let frem = d % (2 * (((Nbaselines + Nant))));
        if (trem == 0) {
            t += 1;
            freq_idx = -1;
        }
        if (frem == 0) {
            freq_idx += 1;
        }
        data.push({x: uv_points[d], y: uv_points[d+1], freq: freqs[freq_idx]});
        data.push({x: -uv_points[d], y: -uv_points[d+1], freq: freqs[freq_idx]});
    });
    */
    d3.range(0, uv_points.length, 2).forEach(i => {
        freq_idx = (i % (2 * freq_channels)) / 2;
        data.push({x: uv_points[i], y: uv_points[i+1], freq: freqs[freq_idx]});
        data.push({x: -uv_points[i], y: -uv_points[i+1], freq: freqs[freq_idx]});
    });

    if (colour_points.checked) {
        const freqColorScale = d3
          .scaleSequential()
          //.domain([freq_value + 50e6, freq_value])
          .domain([freq_value, freq_value + 50e6])
          .interpolator(d3.interpolateSpectral);
          //.interpolator(d3.interpolateYlGnBu);
          //.interpolator(d3.interpolateRdYlGn);

        let fillColor = fc
          .webglFillColor()
          .value(d => webglColor(freqColorScale(d.freq)))
          .data(data);

        pointSeries.decorate(program => fillColor(program));
    } else {
        let fillColor = fc
          .webglFillColor()
          .value(d => [0, 0, 0, 1])
          .data(data);

        pointSeries.decorate(program => fillColor(program));
    }
    render();
    return uv_points.length / 2;
}

function updatePlot() {
    status.innerText = `Rendering ...`;
    const start = performance.now();
    let Npoints = updatePlotUvCoverage();
    const end = performance.now();
    if (Npoints) {
        const array = document.querySelector("input[name=array]:checked");
        let telescope = document.querySelector(`label[for=${array.id}]`).innerHTML;
        status.innerText = `UV points rendered: ${Npoints}\nRender time: ${Math.ceil(end - start)}ms`;
    } else {
        status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;
    }
}
