export interface FoundApplications {
  paths: ApplicationPath[];
  commonPathPrefix: string;
}

export interface ApplicationPath {
  absolute: string;
  stripped: string;
}

export default {};
