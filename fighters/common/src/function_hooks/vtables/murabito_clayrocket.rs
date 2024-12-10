use super::*;
use super::energy::PaddedVec2;

#[skyline::hook(offset = 0x3463300)]
unsafe extern "C" fn murabito_clayrocket_kinetic_type_handler(
    _vtable: u64,
    kinetic_type: i32,
    module_accessor: *mut BattleObjectModuleAccessor
) {
    if kinetic_type == *WEAPON_KINETIC_TYPE_MURABITO_CLAYROCKET_FALL {
        // let sum_speed = KineticModule::get_sum_speed(module_accessor, 1);
        KineticModule::unable_energy_all(module_accessor);

        // let fall_decel = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("fall_decel"));
        let fall_gravity = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("fall_gravity"));
        let fall_limit_gravity = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("fall_limit_gravity"));

        // let kinetic_energy = KineticModule::get_energy(module_accessor, 0) as *mut super::energy::KineticEnergy;
        // (*kinetic_energy).enable = true;
        // (*kinetic_energy).accel = PaddedVec2::zeros();
        // (*kinetic_energy).speed_max = PaddedVec2::zeros();
        // (*kinetic_energy).speed_brake = PaddedVec2::new(fall_decel, 0.0);
        // (*kinetic_energy).speed_limit = PaddedVec2::new(-(*kinetic_energy).speed_limit.x, 0.0);

        let kinetic_energy = KineticModule::get_energy(module_accessor, 2) as *mut super::energy::KineticEnergy;
        (*kinetic_energy).enable = true;
        (*kinetic_energy).x34 = -fall_gravity;
        (*kinetic_energy).x38 = fall_limit_gravity;
    }
    else if kinetic_type == *WEAPON_KINETIC_TYPE_MURABITO_CLAYROCKET_FLY {
        let control_speed = WorkModule::get_int(module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_INT_CONTROL_SPEED);
        let ground = WorkModule::is_flag(module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_GROUND);
        let ride = WorkModule::is_flag(module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_RIDE);
        let kinetic_type = KineticModule::get_kinetic_type(module_accessor);
        let speed = if kinetic_type == *WEAPON_KINETIC_TYPE_MURABITO_CLAYROCKET_FLY {
            KineticModule::get_sum_speed(module_accessor, 2)
        }
        else {
            let param = if ride {
                hash40("ride_init_speed")
            }
            else {
                hash40("fly_speed")
            };
            WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), param)
        };

        KineticModule::unable_energy_all(module_accessor);

        let param = if ride {
            hash40("ride_accel")
        }
        else {
            hash40("fly_accel")
        };
        let accel = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), param);

        let param = if ground {
            hash40("fly_limit_speed_ground")
        }
        else {
            hash40("fly_limit_speed_air")
        };
        let mut limit = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), param);

        if ride {
            let param = if control_speed == 2 {
                hash40("ride_limit_speed_mul_negative")
            }
            else if control_speed == 1 {
                hash40("ride_limit_speed_mul_positive")
            }
            else if control_speed == 0 {
                hash40("ride_limit_speed_mul")
            }
            else {
                0
            };
            if param != 0 {
                limit *= WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), param);
            }
        }

        let speed_max = if WorkModule::is_flag(module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_GET_OFF)
        && speed <= limit {
            speed
        }
        else {
            limit
        };

        let kinetic_energy = KineticModule::get_energy(module_accessor, 2) as *mut super::energy::KineticEnergy;

        (*kinetic_energy).x38 = speed_max;
        (*kinetic_energy).enable = true;
        (*kinetic_energy).speed = PaddedVec2::new(0.0, speed);
        (*kinetic_energy).x34 = accel;
    }
    else if kinetic_type == 2 {
        KineticModule::unable_energy_all(module_accessor);
    }
}

pub fn install() {
    skyline::install_hooks!(
        murabito_clayrocket_kinetic_type_handler
    );
}