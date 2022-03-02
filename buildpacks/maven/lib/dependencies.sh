#!/usr/bin/env bash

dependencies::has_spring_boot() {
	local -r app_directory=${1:?}
	local -r dependency_list_path="${app_directory}/target/mvn-dependency-list.log"

	if [[ -f "${dependency_list_path}" ]]; then
		grep -q "org.springframework.boot:spring-boot" "${dependency_list_path}"
	fi
}

dependencies::has_wildfly_swarm() {
	local -r app_directory=${1:?}
	local -r pom_path="${app_directory}/pom.xml"

	if [[ -f "${pom_path}" ]]; then
		grep -q "<groupId>org.wildfly.swarm" "${pom_path}"
	fi
}

dependencies::app_requires_postgres() {
	local -r app_directory=${1:?}
	local -r pom_path="${app_directory}/pom.xml"

	if [[ -f "${pom_path}" ]]; then
		grep -q "<groupId>org.postgresql" "${pom_path}" ||
			grep -q "<groupId>postgresql" "${pom_path}" ||
			grep -q "<groupId>com.impossibl.pgjdbc-ng" "${pom_path}"
	fi
}
