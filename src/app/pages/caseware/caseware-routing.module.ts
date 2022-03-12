import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { CasewareComponent } from './caseware.component';

const routes: Routes = [
  { path: '', component: CasewareComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class CasewareRoutingModule { }
