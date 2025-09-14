use nalgebra::{DMatrix, Isometry3, Point3, Translation3, UnitQuaternion, Vector3};
use rapier3d::prelude::*;

use crate::*;

pub struct Ocean {}
impl Ocean {
    const HITBOX_SIZE: f32 = 1000.0;
    const WATER_HEIGHT: f32 = -2.96968;
}
impl GameObject for Ocean {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("ocean.obj");

    fn get_collision_box() -> ColliderBuilder {
        ColliderBuilder::heightfield(
            DMatrix::from_element(2, 2, Self::WATER_HEIGHT),
            Vector3::new(Self::HITBOX_SIZE, 1.0, Self::HITBOX_SIZE),
        )
    }
}

pub struct Ground {}
impl GameObject for Ground {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("ground.obj");

    #[rustfmt::skip]
    const debug_lines: &'static [RawDebugLine] = &[
        // origin
        RawDebugLine { col: RED, pos1: [0.0, 0.0, 0.0], pos2: [1.0, 0.0, 0.0], },
        RawDebugLine { col: GREEN, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 1.0, 0.0], },
        RawDebugLine { col: BLUE, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 0.0, 1.0],  },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [-1.0, 0.0, 0.0], },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [0.0, -1.0, 0.0], },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 0.0, -1.0], },
    ];

    fn get_collision_box() -> ColliderBuilder {
        let hitbox_mesh: RawMesh = load_obj_mesh!("ground_hitbox.obj")[0];
        let mut verts: Vec<Point3<f32>> = Vec::new();
        let mut indices: Vec<[u32; 3]> = Vec::new();

        for v in hitbox_mesh.verts {
            verts.push(Point3::from(v.pos));
        }

        let mut count = 0;
        let mut cur_face: [u32; 3] = [0; 3];
        for index in hitbox_mesh.indices {
            cur_face[count] = *index as u32;
            count += 1;
            if count == 3 {
                indices.push(cur_face);
                count = 0;
            }
        }
        ColliderBuilder::trimesh(verts, indices).expect("Failed to create trimesh collision box")
    }
}

pub struct Roads {}
impl GameObject for Roads {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("roads.obj");
}

pub struct Buildings {}
impl Buildings {}
impl GameObject for Buildings {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("buildings.obj");

    fn get_collision_box() -> ColliderBuilder {
        let hitbox_mesh: RawMesh = load_obj_mesh!("buildings_hitbox.obj")[0];
        let mut verts: Vec<Point3<f32>> = Vec::new();
        let mut indices: Vec<[u32; 3]> = Vec::new();

        for v in hitbox_mesh.verts {
            verts.push(Point3::from(v.pos));
        }

        let mut count = 0;
        let mut cur_face: [u32; 3] = [0; 3];
        for index in hitbox_mesh.indices {
            cur_face[count] = *index as u32;
            count += 1;
            if count == 3 {
                indices.push(cur_face);
                count = 0;
            }
        }
        ColliderBuilder::trimesh(verts, indices).expect("Failed to create trimesh collision box")
    }
}

pub struct Streetlights {}
impl GameObject for Streetlights {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("streetlight.obj");
    fn get_collision_box() -> ColliderBuilder {
        ColliderBuilder::compound(
            Self::get_instances()
                .into_iter()
                .map(|transform| (transform, SharedShape::cylinder(5.95, 0.145)))
                .collect(),
        )
    }

    #[rustfmt::skip]
    fn get_instances() -> Vec<Isometry3<f32>> {
        vec![
            Isometry3::from_parts(Translation3::new(-175.370361328125,3.1805992126464844,-29.002857208251953), UnitQuaternion::new(Vector3::y() * 45.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-180.74639892578125,3.1805992126464844,-13.614952087402344), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-68.95471954345703,3.1805992126464844,-34.44168472290039), UnitQuaternion::new(Vector3::y() * 0.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-144.16598510742188,3.1805992126464844,197.72842407226562), UnitQuaternion::new(Vector3::y() * 157.94998168945312f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-196.50210571289062,3.1805992126464844,9.894065856933594), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-86.22615051269531,3.1805992126464844,-34.44168472290039), UnitQuaternion::new(Vector3::y() * 0.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-105.18698120117188,3.1805992126464844,-34.44168472290039), UnitQuaternion::new(Vector3::y() * 0.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-122.92756652832031,3.1805992126464844,-34.44168472290039), UnitQuaternion::new(Vector3::y() * 0.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-144.8826446533203,3.1805992126464844,-34.44168472290039), UnitQuaternion::new(Vector3::y() * 0.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-160.5371856689453,3.1805992126464844,-34.44168472290039), UnitQuaternion::new(Vector3::y() * 0.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-17.644187927246094,3.1805992126464844,-28.631267547607422), UnitQuaternion::new(Vector3::y() * -45.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-32.65501403808594,3.1805992126464844,-34.44168472290039), UnitQuaternion::new(Vector3::y() * 0.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-48.26963806152344,3.1805992126464844,-34.44168472290039), UnitQuaternion::new(Vector3::y() * 0.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-196.50210571289062,3.1805992126464844,29.982955932617188), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-196.50210571289062,3.1805992126464844,49.96833038330078), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-196.50210571289062,3.1805992126464844,90.31975555419922), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-196.50210571289062,3.1805992126464844,70.01409149169922), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-196.50210571289062,3.1805992126464844,110.40863800048828), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-196.50210571289062,3.1805992126464844,130.39401245117188), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-196.50210571289062,3.1805992126464844,150.43978881835938), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-126.06938171386719,3.1805992126464844,16.5325927734375), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-126.06938171386719,3.1805992126464844,36.621482849121094), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-126.06938171386719,3.1805992126464844,56.60686492919922), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-126.06938171386719,3.1805992126464844,96.9582748413086), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-126.06938171386719,3.1805992126464844,76.6526107788086), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-126.06938171386719,3.1805992126464844,117.04717254638672), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-126.06938171386719,3.1805992126464844,137.03253173828125), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-126.06938171386719,3.1805992126464844,-3.9374160766601562), UnitQuaternion::new(Vector3::y() * 90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-14.15191650390625,3.1805992126464844,76.96678924560547), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-14.15191650390625,3.1805992126464844,117.3182144165039), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-14.15191650390625,3.1805992126464844,97.0125503540039), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-14.15191650390625,3.1805992126464844,137.4071044921875), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-14.15191650390625,3.1805992126464844,157.3924560546875), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-14.15191650390625,3.1805992126464844,177.438232421875), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-14.15191650390625,3.1805992126464844,197.42837524414062), UnitQuaternion::new(Vector3::y() * -90.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-160.70880126953125,3.1805992126464844,190.17245483398438), UnitQuaternion::new(Vector3::y() * 157.94998168945312f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-177.30165100097656,3.1805992126464844,182.59359741210938), UnitQuaternion::new(Vector3::y() * 157.94998168945312f32.to_radians())),
            Isometry3::from_parts(Translation3::new(50.726383209228516,3.0241029262542725,272.9693603515625), UnitQuaternion::new(Vector3::y() * -176.42002868652344f32.to_radians())),
            Isometry3::from_parts(Translation3::new(30.41982078552246,3.0241029262542725,272.8839416503906), UnitQuaternion::new(Vector3::y() * -188.76002502441406f32.to_radians())),
            Isometry3::from_parts(Translation3::new(10.675113677978516,3.0241029262542725,268.33599853515625), UnitQuaternion::new(Vector3::y() * 157.94998168945312f32.to_radians())),
            Isometry3::from_parts(Translation3::new(72.21736907958984,3.0241029262542725,268.468505859375), UnitQuaternion::new(Vector3::y() * -157.77003479003906f32.to_radians())),
            Isometry3::from_parts(Translation3::new(92.34943389892578,3.0241029262542725,259.50384521484375), UnitQuaternion::new(Vector3::y() * -150.42002868652344f32.to_radians())),
            Isometry3::from_parts(Translation3::new(110.37040710449219,3.0241029262542725,242.489501953125), UnitQuaternion::new(Vector3::y() * -128.26002502441406f32.to_radians())),
            Isometry3::from_parts(Translation3::new(120.05300903320312,3.1805992126464844,223.31692504882812), UnitQuaternion::new(Vector3::y() * -104.6500244140625f32.to_radians())),
            Isometry3::from_parts(Translation3::new(123.82244110107422,3.1805992126464844,204.51962280273438), UnitQuaternion::new(Vector3::y() * -87.35002136230469f32.to_radians())),
            Isometry3::from_parts(Translation3::new(121.04767608642578,3.1805992126464844,184.6693115234375), UnitQuaternion::new(Vector3::y() * -78.27001953125f32.to_radians())),
            Isometry3::from_parts(Translation3::new(115.15660858154297,3.1805992126464844,168.27679443359375), UnitQuaternion::new(Vector3::y() * -69.59001922607422f32.to_radians())),
            Isometry3::from_parts(Translation3::new(111.02141571044922,3.181950569152832,148.9210205078125), UnitQuaternion::new(Vector3::y() * -80.51001739501953f32.to_radians())),
            Isometry3::from_parts(Translation3::new(110.42385864257812,3.171802520751953,128.6937255859375), UnitQuaternion::new(Vector3::y() * -95.53001403808594f32.to_radians())),
            Isometry3::from_parts(Translation3::new(113.42892456054688,3.170912265777588,109.4302749633789), UnitQuaternion::new(Vector3::y() * -108.58000946044922f32.to_radians())),
            Isometry3::from_parts(Translation3::new(121.30413818359375,3.1820523738861084,90.34078979492188), UnitQuaternion::new(Vector3::y() * -112.64000701904297f32.to_radians())),
            Isometry3::from_parts(Translation3::new(129.99085998535156,3.1962780952453613,73.79931640625), UnitQuaternion::new(Vector3::y() * -125.0300064086914f32.to_radians())),
            Isometry3::from_parts(Translation3::new(158.49380493164062,3.1805992126464844,-260.193115234375), UnitQuaternion::new(Vector3::y() * -38.53001022338867f32.to_radians())),
            Isometry3::from_parts(Translation3::new(140.00758361816406,3.1805992126464844,-274.28155517578125), UnitQuaternion::new(Vector3::y() * -30.08000946044922f32.to_radians())),
            Isometry3::from_parts(Translation3::new(121.17276763916016,3.1805992126464844,-286.4112854003906), UnitQuaternion::new(Vector3::y() * -30.08000946044922f32.to_radians())),
            Isometry3::from_parts(Translation3::new(175.79547119140625,3.1805992126464844,-245.19837951660156), UnitQuaternion::new(Vector3::y() * -38.53001022338867f32.to_radians())),
            Isometry3::from_parts(Translation3::new(190.3779296875,3.1805992126464844,-228.044921875), UnitQuaternion::new(Vector3::y() * -45.070011138916016f32.to_radians())),
            Isometry3::from_parts(Translation3::new(204.04318237304688,3.1805992126464844,-210.83132934570312), UnitQuaternion::new(Vector3::y() * -56.13001251220703f32.to_radians())),
            Isometry3::from_parts(Translation3::new(216.73106384277344,3.1805992126464844,-192.376220703125), UnitQuaternion::new(Vector3::y() * -58.510009765625f32.to_radians())),
            Isometry3::from_parts(Translation3::new(227.88104248046875,3.1805992126464844,-171.99871826171875), UnitQuaternion::new(Vector3::y() * 296.8899841308594f32.to_radians())),
            Isometry3::from_parts(Translation3::new(238.11831665039062,3.1805992126464844,-150.0832977294922), UnitQuaternion::new(Vector3::y() * 296.8899841308594f32.to_radians())),
            Isometry3::from_parts(Translation3::new(246.54908752441406,3.1805992126464844,-128.45977783203125), UnitQuaternion::new(Vector3::y() * 296.8899841308594f32.to_radians())),
            Isometry3::from_parts(Translation3::new(254.3545684814453,3.1805992126464844,-107.72562408447266), UnitQuaternion::new(Vector3::y() * 296.8899841308594f32.to_radians())),
            Isometry3::from_parts(Translation3::new(261.6575012207031,3.1805992126464844,-89.50133514404297), UnitQuaternion::new(Vector3::y() * 296.8899841308594f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-44.534454345703125,3.1805992126464844,-126.59850311279297), UnitQuaternion::new(Vector3::y() * 180.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-61.803306579589844,3.1805992126464844,-126.29997253417969), UnitQuaternion::new(Vector3::y() * 180.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-81.01307678222656,3.1805992126464844,-124.46162414550781), UnitQuaternion::new(Vector3::y() * 180.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-99.00276947021484,3.1805992126464844,-121.1337661743164), UnitQuaternion::new(Vector3::y() * 192.27999877929688f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-6.477795600891113,3.1805992126464844,-127.47770690917969), UnitQuaternion::new(Vector3::y() * 180.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-23.85246467590332,3.1805992126464844,-126.95603942871094), UnitQuaternion::new(Vector3::y() * 180.0f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-115.87124633789062,3.1805992126464844,-114.08425903320312), UnitQuaternion::new(Vector3::y() * 209.85000610351562f32.to_radians())),
            Isometry3::from_parts(Translation3::new(-127.20082092285156,3.1805992126464844,-104.0135269165039), UnitQuaternion::new(Vector3::y() * 224.66000366210938f32.to_radians())),
        ]
    }
}

pub struct Trees1 {}
impl GameObject for Trees1 {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("tree1.obj");

    fn get_collision_box() -> ColliderBuilder {
        ColliderBuilder::compound(
            Self::get_instances()
                .into_iter()
                .map(|transform| (transform, SharedShape::cylinder(3.0, 0.23)))
                .collect(),
        )
    }

    #[rustfmt::skip]
    fn get_instances() -> Vec<Isometry3<f32>> {
        vec![
            Isometry3::from_parts(Translation3::new(-168.0408172607422,3.084705114364624,2.0454177856445312), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-112.27576446533203,3.084705114364624,196.3356475830078), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-18.013534545898438,3.084705114364624,-4.992973327636719), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-27.114788055419922,3.084705114364624,-4.6475830078125), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.63613891601562,3.084705114364624,98.21630859375), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.63613891601562,3.084705114364624,77.91064453125), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.63613891601562,3.084705114364624,107.40689086914062), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.63613891601562,3.084705114364624,136.14610290527344), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(5.148284912109375,3.084705114364624,107.90838623046875), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(5.148284912109375,3.084705114364624,148.2598114013672), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(5.148284912109375,3.084705114364624,127.95414733886719), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.5023422241211,3.084705114364624,45.505714416503906), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.5023422241211,3.084705114364624,25.416824340820312), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-137.37991333007812,3.084705114364624,-95.98348236083984), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-143.5357208251953,3.084705114364624,-75.4641342163086), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-119.83587646484375,3.084705114364624,-80.49138641357422), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.5023422241211,3.084705114364624,16.226242065429688), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.5023422241211,3.084705114364624,36.31513214111328), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.5023422241211,3.084705114364624,56.300514221191406), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-96.49781036376953,3.084705114364624,196.50071716308594), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.63613891601562,3.084705114364624,87.10122680664062), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.63613891601562,3.084705114364624,117.99885559082031), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-108.63613891601562,3.084705114364624,127.18943786621094), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-87.69541931152344,3.084705114364624,196.43365478515625), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-64.58572387695312,3.084705114364624,196.80908203125), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-104.74945068359375,3.084705114364624,196.41036987304688), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-79.44682312011719,3.084705114364624,196.66156005859375), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-71.92049407958984,3.084705114364624,196.7362823486328), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-49.162052154541016,3.084705114364624,196.50071716308594), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-41.42338562011719,3.084705114364624,196.61094665527344), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-19.377410888671875,3.084705114364624,196.80908203125), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-34.06122589111328,3.084705114364624,196.66156005859375), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-26.71218490600586,3.084705114364624,196.7362823486328), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-23.483901977539062,3.084705114364624,-13.782264709472656), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-30.804275512695312,3.084705114364624,-19.201133728027344), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-46.52943801879883,3.084705114364624,-17.843772888183594), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-42.62527847290039,3.084705114364624,-26.072357177734375), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-148.78001403808594,3.084705114364624,-29.524314880371094), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-145.68362426757812,3.084705114364624,-21.199745178222656), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-159.65809631347656,3.084705114364624,-31.0931396484375), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-164.78744506835938,3.084705114364624,-20.677223205566406), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-174.38462829589844,3.084705114364624,-8.575782775878906), UnitQuaternion::<f32>::identity()),
            Isometry3::from_parts(Translation3::new(-175.51593017578125,3.084705114364624,-14.784629821777344), UnitQuaternion::<f32>::identity()),
        ]
    }
}
