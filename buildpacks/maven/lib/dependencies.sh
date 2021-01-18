#!/usr/bin/env bash

dependencies::has_spring_boot() {
	local -r app_directory=${1:?}
	local -r pom_path="${app_directory}/pom.xml"

	[[ -f "${pom_path}" ]] &&
		[[ -n "$(grep "<groupId>org.springframework.boot" "${pom_path}")" ]] &&
		[[ -n "$(grep "<artifactId>spring-boot" "${pom_path}")" ]]
}

dependencies::has_wildfly_swarm() {
	local -r app_directory=${1:?}
	local -r pom_path="${app_directory}/pom.xml"

	[[ -f "${pom_path}" ]] && [[ -n "$(grep "<groupId>org.wildfly.swarm" "${pom_path}")" ]]
}

dependencies::app_requires_postgres() {
	local -r app_directory=${1:?}
	local -r pom_path="${app_directory}/pom.xml"

	[[ -f "${pom_path}" ]] &&
		{
			[[ -n "$(grep "<groupId>org.postgresql" "${pom_path}")" ]] ||
				[[ -n "$(grep "<groupId>postgresql" "${pom_path}")" ]] ||
				[[ -n "$(grep "<groupId>com.impossibl.pgjdbc-ng" "${pom_path}")" ]]
		}
}
