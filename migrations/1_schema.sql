CREATE TYPE site_type AS ENUM (
    'power_plant',
    'road',
    'housing',
    'bridge',
    'park'
);

CREATE TYPE site_risk_level AS ENUM (
    'low',
    'medium',
    'high'
);

CREATE TYPE worker_profession AS ENUM (
    'electrician',
    'plumber',
    'welder',
    'driver',
    'mason'
);

CREATE TYPE technical_qualification AS ENUM (
    'techician',
    'technologist',
    'engineer'
);

CREATE TYPE management_position AS ENUM (
    'master',
    'foreman'
);

CREATE TYPE employee_class AS ENUM (
    'worker',
    'technical_personnel'
);

CREATE TYPE gender AS ENUM (
    'male',
    'female'
);

CREATE TABLE employee (
    id SERIAL PRIMARY KEY,
    class employee_class NOT NULL,
    first_name VARCHAR(32) NOT NULL,
    last_name VARCHAR(32) NOT NULL,
    middle_name VARCHAR(32),
    gender gender NOT NULL,
    photo VARCHAR(256),
    salary INTEGER NOT NULL,
    phone_number VARCHAR(16) NOT NULL
);

CREATE TABLE worker (
    id INTEGER UNIQUE NOT NULL REFERENCES employee(id),
    profession worker_profession NOT NULL,
    union_name VARCHAR(256)
);

CREATE TABLE electrician (
    id INTEGER UNIQUE NOT NULL REFERENCES worker(id),
    voltage_specialization VARCHAR(64) NOT NULL
);

CREATE TABLE plumber (
    id INTEGER UNIQUE NOT NULL REFERENCES worker(id),
    pipe_specializaiton VARCHAR(64) NOT NULL
);

CREATE TABLE welder (
    id INTEGER UNIQUE NOT NULL REFERENCES worker(id),
    welding_machine VARCHAR(64) NOT NULL
);

CREATE TABLE driver  (
    id INTEGER UNIQUE NOT NULL REFERENCES worker(id),
    vehicle_type VARCHAR(64) NOT NULL,
    number_of_accidents INTEGER NOT NULL
);

CREATE TABLE mason (
    id INTEGER UNIQUE NOT NULL REFERENCES worker(id),
    hq_restoration_skills BOOLEAN NOT NULL
);

CREATE TABLE technical_personnel (
    id INTEGER UNIQUE NOT NULL REFERENCES employee(id),
    qualification technical_qualification NOT NULL,
    position management_position,
    education_level VARCHAR(64) NOT NULL,
    software_skills VARCHAR(256),
    is_project_manager BOOLEAN NOT NULL
);

CREATE TABLE technician (
    id INTEGER UNIQUE NOT NULL REFERENCES technical_personnel(id),
    safety_training_level VARCHAR(64) NOT NULL,
);

CREATE TABLE technologist (
    id INTEGER UNIQUE NOT NULL REFERENCES technical_personnel(id),
    management_tools VARCHAR(256) NOT NULL,
);

CREATE TABLE engineer (
    id INTEGER UNIQUE NOT NULL REFERENCES technical_personnel(id),
    pe_license_id BIGINT NOT NULL,
);

CREATE TABLE brigade (
    id SERIAL PRIMARY KEY,
    brigadier_id INTEGER NOT NULL REFERENCES worker(id)
);

CREATE TABLE assignment (
    brigade_id INTEGER NOT NULL REFERENCES brigade(id),
    worker_id INTEGER NOT NULL REFERENCES worker(id)
);

CREATE TABLE department (
    id SERIAL PRIMARY KEY,
    supervisor_id INTEGER NOT NULL REFERENCES technical_personnel(id),
    name VARCHAR(128)
);

CREATE TABLE area (
    id SERIAL PRIMARY KEY,
    department_id INTEGER NOT NULL REFERENCES department(id),
    supervisor_id INTEGER NOT NULL REFERENCES technical_personnel(id),
    name VARCHAR(128)
);

CREATE TABLE client (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    inn BIGINT NOT NULL,
    address VARCHAR(128) NOT NULL,
    contact_person_name VARCHAR(64) NOT NULL,
    contact_person_email VARCHAR(128) NOT NULL,
    is_vip BOOLEAN NOT NULL
);

CREATE TABLE site (
    id SERIAL PRIMARY KEY,
    area_id INTEGER NOT NULL REFERENCES area(id),
    client_id INTEGER NOT NULL REFERENCES client(id),
    type site_type NOT NULL,
    location POINT NOT NULL,
    risk_level site_risk_level NOT NULL,
    description TEXT
);

CREATE TABLE power_plant (
    site_id INTEGER NOT NULL REFERENCES site(id),
    energy_output INTEGER NOT NULL,
    energy_source VARCHAR(64) NOT NULL,
    is_grid_connected BOOLEAN NOT NULL
);

CREATE TABLE road (
    site_id INTEGER NOT NULL REFERENCES site(id),
    length INTEGER NOT NULL,
    lanes INTEGER NOT NULL,
    surface VARCHAR(64) NOT NULL
);

CREATE TABLE housing (
    site_id INTEGER NOT NULL REFERENCES site(id),
    number_of_floors INTEGER NOT NULL,
    number_of_entrances INTEGER NOT NULL,
    housing_type VARCHAR(64) NOT NULL,
    energy_efficiency CHAR
);

CREATE TABLE bridge (
    site_id INTEGER NOT NULL REFERENCES site(id),
    length INTEGER NOT NULL,
    road_material VARCHAR(64) NOT NULL,
    max_load INTEGER NOT NULL
);

CREATE TABLE park (
    site_id INTEGER NOT NULL REFERENCES site(id),
    area REAL NOT NULL,
    has_playground BOOLEAN NOT NULL,
    has_lighting BOOLEAN NOT NULL
);

CREATE TABLE task (
    id SERIAL PRIMARY KEY,
    site_id INTEGER NOT NULL REFERENCES site(id),
    brigade_id INTEGER REFERENCES brigade(id),
    period_start DATE NOT NULL,
    expected_period_end DATE NOT NULL,
    actual_period_end DATE,
    name VARCHAR(64) NOT NULL,
    description TEXT
);

CREATE TABLE material (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    cost REAL NOT NULL,
    units VARCHAR(32) NOT NULL
);

CREATE TABLE expenditure (
    task_id INTEGER NOT NULL REFERENCES task(id),
    material_id INTEGER NOT NULL REFERENCES material(id),
    expected_amount INTEGER NOT NULL,
    actual_amount INTEGER
);

CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    amount INTEGER NOT NULL,
    purchase_date DATE NOT NULL,
    purchase_cost INTEGER NOT NULL,
    fuel_type VARCHAR(32)
);

CREATE TABLE equipment_allocation (
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    department_id INTEGER NOT NULL REFERENCES department(id),
    site_id INTEGER REFERENCES site(id),
    amount INTEGER NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL
);
