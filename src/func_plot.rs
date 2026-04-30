use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;

use nalgebra::Vector3;
use ndarray::Array1;

#[allow(clippy::too_many_arguments)]
pub fn draw_uvcoverage(
    dec: f64,
    nu: f64,
    n_chan: u32,
    phi: f64,
    duration: f64,
    n_times: usize,
    array_name: &str,
    antenna_mask: Vec<u8>,
) -> (Vec<f64>, Vec<f64>) {
    let mut vec_u = Vec::<f64>::new();
    let mut vec_v = Vec::<f64>::new();
    let channel_width = if array_name == "LOFAR" {
        5e6
    } else if array_name == "e-MERLIN" {
        5e7
    } else {
        5e6
    };

    run_discrete(
        &mut vec_u,
        &mut vec_v,
        &channel_width,
        &nu,
        &(nu + (n_chan as f64 * channel_width)),
        &161.75_f64.to_radians(),
        &dec,
        &duration,
        &n_times,
        &phi,
        array_name,
        antenna_mask,
    );
    (vec_u, vec_v)
}

// Celestial pole
const XYZ_NORTH: Vector3<f64> = Vector3::new(0.0, 0.0, 1.0);

#[allow(unused)]
fn read_array_from_csv(filename: &str) -> Result<BTreeMap<String, Vector3<f64>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut ants = BTreeMap::<String, Vector3<f64>>::new();
    for result in reader.records() {
        let record = result?;
        let name: String = record[0].to_string();
        let x: f64 = record[1].parse::<f64>()?;
        let y: f64 = record[2].parse::<f64>()?;
        let z: f64 = record[3].parse::<f64>()?;
        let pos = Vector3::<f64>::new(x, y, z);
        ants.insert(name, pos);
    }
    Ok(ants)
}

#[allow(clippy::too_many_arguments)]
pub fn run_discrete(
    vec_u: &mut Vec<f64>,
    vec_v: &mut Vec<f64>,
    channel_width: &f64,
    band_low: &f64,
    band_high: &f64,
    ra: &f64,
    dec: &f64,
    duration: &f64,
    phi_resolution: &usize,
    phi_start: &f64,
    array_name: &str,
    antenna_filter: Vec<u8>,
) {
    //let ants = read_array_from_csv(telescope_configuration)
    //    .expect("Failed to read telescope configuration.");
    let mut ants = get_array_from_name(array_name);
    let mut keys_to_remove: Vec<String> = Vec::<String>::new();
    for (af, key) in antenna_filter.iter().zip(ants.keys()) {
        if *af == 0 {
            keys_to_remove.push(key.to_string());
        }
    }
    ants.retain(|k, _| !keys_to_remove.contains(k));

    // Observe around noon, i.e. 12 h or pi +/- half the observation.
    let start_phi = ra + phi_start * (std::f64::consts::PI / 12.0);
    let phi_scan = duration * (std::f64::consts::PI / 12.0);
    let phi_range = Array1::linspace(start_phi, start_phi + phi_scan, *phi_resolution);
    let freqs: Array1<f64> =
        Array1::<f64>::range(*band_low, *band_high, *channel_width) / 299792458.0;

    for phi in phi_range.into_iter() {
        // This is s
        let xyz_pointing = Vector3::new(
            (phi - ra).cos() * dec.cos(),
            -(phi - ra).sin() * dec.cos(),
            dec.sin(),
        );
        // North: N = (X, Y, Z) = (0, 0, 1)
        // This is "N - s"
        let xyz_fake_north_projected = (XYZ_NORTH - xyz_pointing).normalize();
        // This is u
        //let xyz_east_projected = xyz_pointing.cross(&xyz_fake_north_projected).normalize();
        let xyz_east_projected = xyz_fake_north_projected.cross(&xyz_pointing).normalize();
        // This is v
        let xyz_north_projected = xyz_pointing.cross(&xyz_east_projected).normalize();
        //assert!(xyz_fake_north_projected != xyz_north_projected);

        ants.iter().enumerate().for_each(|ant1| {
            let (idx_ant1, (_ant1, ant1_pos)) = ant1;
            ants.iter().skip(idx_ant1).for_each(|(_ant2, ant2_pos)| {
                let xyz_baseline = ant1_pos - ant2_pos;

                let xyz_baseline_projected =
                    xyz_baseline - xyz_baseline.dot(&xyz_pointing) * xyz_pointing;

                let baseline_comp_ns = xyz_baseline_projected.dot(&xyz_north_projected);
                let baseline_comp_ew = xyz_baseline_projected.dot(&xyz_east_projected);
                for nu in &freqs {
                    let uv_ns = baseline_comp_ns * nu;
                    let uv_ew = baseline_comp_ew * nu;
                    vec_u.push(uv_ew);
                    vec_v.push(uv_ns);
                }
            });
        });
    }
}

fn get_array_from_name(name: &str) -> BTreeMap<String, Vector3<f64>> {
    if name == "LOFAR" {
        BTreeMap::<String, Vector3<f64>>::from([
            (
                "CS001HBA0".to_string(),
                Vector3::new(3826896.25, 460979.47, 5064658.00),
            ),
            (
                "CS001HBA1".to_string(),
                Vector3::new(3826979.50, 460897.59, 5064603.00),
            ),
            (
                "CS002HBA0".to_string(),
                Vector3::new(3826601.00, 460953.41, 5064881.00),
            ),
            (
                "CS002HBA1".to_string(),
                Vector3::new(3826565.50, 460958.12, 5064907.50),
            ),
            (
                "CS003HBA0".to_string(),
                Vector3::new(3826471.25, 461000.12, 5064974.00),
            ),
            (
                "CS003HBA1".to_string(),
                Vector3::new(3826517.75, 461035.25, 5064936.00),
            ),
            (
                "CS004HBA0".to_string(),
                Vector3::new(3826585.75, 460865.84, 5064900.50),
            ),
            (
                "CS004HBA1".to_string(),
                Vector3::new(3826579.50, 460917.47, 5064900.50),
            ),
            (
                "CS005HBA0".to_string(),
                Vector3::new(3826701.25, 460989.25, 5064802.50),
            ),
            (
                "CS005HBA1".to_string(),
                Vector3::new(3826631.25, 461021.81, 5064852.50),
            ),
            (
                "CS006HBA0".to_string(),
                Vector3::new(3826653.75, 461136.44, 5064825.00),
            ),
            (
                "CS006HBA1".to_string(),
                Vector3::new(3826612.50, 461080.31, 5064861.00),
            ),
            (
                "CS007HBA0".to_string(),
                Vector3::new(3826478.75, 461083.72, 5064961.00),
            ),
            (
                "CS007HBA1".to_string(),
                Vector3::new(3826538.00, 461169.72, 5064909.00),
            ),
            (
                "CS011HBA0".to_string(),
                Vector3::new(3826637.50, 461227.34, 5064829.00),
            ),
            (
                "CS011HBA1".to_string(),
                Vector3::new(3826649.00, 461354.25, 5064809.00),
            ),
            (
                "CS013HBA0".to_string(),
                Vector3::new(3826319.00, 460856.12, 5065102.00),
            ),
            (
                "CS013HBA1".to_string(),
                Vector3::new(3826402.00, 460774.28, 5065047.00),
            ),
            (
                "CS017HBA0".to_string(),
                Vector3::new(3826405.00, 461507.47, 5064978.00),
            ),
            (
                "CS017HBA1".to_string(),
                Vector3::new(3826499.75, 461552.50, 5064903.00),
            ),
            (
                "CS021HBA0".to_string(),
                Vector3::new(3826463.50, 460533.09, 5065022.50),
            ),
            (
                "CS021HBA1".to_string(),
                Vector3::new(3826368.75, 460488.06, 5065098.00),
            ),
            (
                "CS024HBA0".to_string(),
                Vector3::new(3827218.25, 461403.91, 5064379.00),
            ),
            (
                "CS024HBA1".to_string(),
                Vector3::new(3827123.50, 461358.88, 5064454.00),
            ),
            (
                "CS028HBA0".to_string(),
                Vector3::new(3825573.25, 461324.59, 5065619.00),
            ),
            (
                "CS028HBA1".to_string(),
                Vector3::new(3825656.25, 461242.75, 5065564.00),
            ),
            (
                "CS030HBA0".to_string(),
                Vector3::new(3826041.50, 460323.38, 5065357.50),
            ),
            (
                "CS030HBA1".to_string(),
                Vector3::new(3825958.50, 460405.22, 5065412.50),
            ),
            (
                "CS031HBA0".to_string(),
                Vector3::new(3826383.00, 460279.34, 5065106.00),
            ),
            (
                "CS031HBA1".to_string(),
                Vector3::new(3826477.75, 460324.38, 5065030.50),
            ),
            (
                "CS032HBA0".to_string(),
                Vector3::new(3826864.25, 460451.94, 5064730.00),
            ),
            (
                "CS032HBA1".to_string(),
                Vector3::new(3826947.50, 460370.06, 5064675.00),
            ),
            (
                "CS101HBA0".to_string(),
                Vector3::new(3825900.00, 461698.91, 5065339.00),
            ),
            (
                "CS101HBA1".to_string(),
                Vector3::new(3825805.25, 461653.88, 5065414.50),
            ),
            (
                "CS103HBA0".to_string(),
                Vector3::new(3826331.50, 462759.06, 5064919.50),
            ),
            (
                "CS103HBA1".to_string(),
                Vector3::new(3826248.50, 462840.94, 5064974.50),
            ),
            (
                "CS201HBA0".to_string(),
                Vector3::new(3826679.25, 461855.25, 5064741.50),
            ),
            (
                "CS201HBA1".to_string(),
                Vector3::new(3826690.75, 461982.12, 5064721.00),
            ),
            (
                "CS301HBA0".to_string(),
                Vector3::new(3827442.50, 461050.81, 5064242.50),
            ),
            (
                "CS301HBA1".to_string(),
                Vector3::new(3827431.00, 460923.91, 5064262.50),
            ),
            (
                "CS302HBA0".to_string(),
                Vector3::new(3827973.25, 459728.62, 5063975.50),
            ),
            (
                "CS302HBA1".to_string(),
                Vector3::new(3827890.00, 459810.47, 5064030.50),
            ),
            (
                "CS401HBA0".to_string(),
                Vector3::new(3826795.75, 460158.91, 5064809.00),
            ),
            (
                "CS401HBA1".to_string(),
                Vector3::new(3826784.25, 460032.00, 5064829.00),
            ),
            (
                "CS501HBA0".to_string(),
                Vector3::new(3825568.75, 460647.62, 5065683.00),
            ),
            (
                "CS501HBA1".to_string(),
                Vector3::new(3825663.50, 460692.66, 5065608.00),
            ),
            (
                "RS106HBA".to_string(),
                Vector3::new(3829205.50, 469142.53, 5062181.00),
            ),
            (
                "RS205HBA".to_string(),
                Vector3::new(3831479.75, 463487.53, 5060990.00),
            ),
            (
                "RS208HBA".to_string(),
                Vector3::new(3847753.25, 466962.81, 5048397.00),
            ),
            (
                "RS210HBA".to_string(),
                Vector3::new(3877827.50, 467536.59, 5025445.50),
            ),
            (
                "RS305HBA".to_string(),
                Vector3::new(3828732.75, 454692.41, 5063850.50),
            ),
            (
                "RS306HBA".to_string(),
                Vector3::new(3829771.25, 452761.69, 5063243.00),
            ),
            (
                "RS307HBA".to_string(),
                Vector3::new(3837964.50, 449627.25, 5057357.50),
            ),
            (
                "RS310HBA".to_string(),
                Vector3::new(3845376.25, 413616.56, 5054796.50),
            ),
            (
                "RS406HBA".to_string(),
                Vector3::new(3818425.00, 452020.28, 5071817.50),
            ),
            (
                "RS407HBA".to_string(),
                Vector3::new(3811649.50, 453459.91, 5076729.00),
            ),
            (
                "RS409HBA".to_string(),
                Vector3::new(3824812.50, 426130.34, 5069252.00),
            ),
            (
                "RS503HBA".to_string(),
                Vector3::new(3824138.50, 459476.97, 5066858.50),
            ),
            (
                "RS508HBA".to_string(),
                Vector3::new(3797136.50, 463114.44, 5086651.50),
            ),
            (
                "RS509HBA".to_string(),
                Vector3::new(3783537.50, 450130.06, 5097866.00),
            ),
            (
                "DE601HBA".to_string(),
                Vector3::new(4034101.50, 487012.75, 4900230.50),
            ),
            (
                "DE602HBA".to_string(),
                Vector3::new(4152568.00, 828789.12, 4754362.00),
            ),
            (
                "DE603HBA".to_string(),
                Vector3::new(3940295.75, 816722.88, 4932394.50),
            ),
            (
                "DE604HBA".to_string(),
                Vector3::new(3796379.75, 877614.12, 5032712.50),
            ),
            (
                "DE605HBA".to_string(),
                Vector3::new(4005718.00, 451028.47, 4926424.50),
            ),
            (
                "FR606HBA".to_string(),
                Vector3::new(4324016.50, 165545.53, 4670271.50),
            ),
            (
                "SE607HBA".to_string(),
                Vector3::new(3370271.75, 712125.88, 5349991.00),
            ),
            (
                "UK608HBA".to_string(),
                Vector3::new(4008462.00, -100376.61, 4943717.00),
            ),
            (
                "DE609HBA".to_string(),
                Vector3::new(3727217.75, 655109.19, 5117003.00),
            ),
            (
                "PL610HBA".to_string(),
                Vector3::new(3738462.50, 1148244.38, 5021710.50),
            ),
            (
                "PL611HBA".to_string(),
                Vector3::new(3850981.00, 1438994.88, 4860499.00),
            ),
            (
                "PL612HBA".to_string(),
                Vector3::new(3551481.75, 1334203.62, 5110157.50),
            ),
            (
                "IE613HBA".to_string(),
                Vector3::new(3801692.00, -528983.94, 5076958.00),
            ),
            (
                "LV614HBA".to_string(),
                Vector3::new(3183249.20, 1276801.80, 5359470.00),
            ),
            (
                "IT".to_string(),
                Vector3::new(4542294.0, 901719.0, 4375397.0),
            ),
            (
                "BG".to_string(),
                Vector3::new(4769182.0, 1680775.0, 3875504.0),
            ),
            (
                "GMRT".to_string(),
                Vector3::new(6020382.0, 2004128.0, 572806.0),
            ),
            (
                "IT-NOTO".to_string(),
                Vector3::new(4934537.0, 1320950.0, 3806510.0),
            ),
            (
                "CZ".to_string(),
                Vector3::new(4074598.0, 1037232.0, 4781393.0),
            ),
            (
                "CZ-Ondrejov".to_string(),
                Vector3::new(3979757.0, 1049928.0, 4856674.0),
            ),
        ])
    } else if name == "e-MERLIN" {
        BTreeMap::<String, Vector3<f64>>::from([
            (
                "Lovell".to_string(),
                Vector3::new(3822626.04, -154105.65, 5086486.04),
            ),
            (
                "MarkII".to_string(),
                Vector3::new(3823009.2144362, -154182.08214624, 5085973.71146964),
            ),
            (
                "Defford".to_string(),
                Vector3::new(3923442.566, -146914.33, 5009755.125),
            ),
            (
                "Knockin".to_string(),
                Vector3::new(3860084.898, -202105.039, 5056568.848),
            ),
            (
                "Pickmere".to_string(),
                Vector3::new(3817549.956, -163031.141, 5089896.654),
            ),
            (
                "Darnhall".to_string(),
                Vector3::new(3829087.899, -169568.955, 5081082.346),
            ),
            (
                "Cambridge".to_string(),
                Vector3::new(3920356.15, 2542.02, 5014284.42),
            ),
        ])
    } else if name == "LAMBDA" {
        BTreeMap::<String, Vector3<f64>>::from([
            (
                "Ceduna".to_string(),
                Vector3::new(-3753358.002833, 3912598.353281, -3348282.527753),
            ),
            (
                "Parkes".to_string(),
                Vector3::new(-4553960.834207, 2816977.410996, -3454172.139680),
            ),
            (
                "Narrabri".to_string(),
                Vector3::new(-4752008.072047, 2791332.201778, -3200197.644361),
            ),
            (
                "Hobart".to_string(),
                Vector3::new(-3950463.657493, 2522542.715740, -4311293.958135),
            ),
            (
                "Perth".to_string(),
                Vector3::new(-2369157.784498, 4881255.883909, -3341632.770807),
            ),
        ])
    } else {
        BTreeMap::<String, Vector3<f64>>::from([
            (
                "PL611HBA".to_string(),
                Vector3::new(3850981.00, 1438994.88, 4860499.00),
            ),
            (
                "IE613HBA".to_string(),
                Vector3::new(3801692.00, -528983.94, 5076958.00),
            ),
        ])
    }
}
