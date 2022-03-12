import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CasewareRoutingModule } from './caseware-routing.module';
import { CasewareComponent } from './caseware.component';
import { NzImageModule } from 'ng-zorro-antd/image';
import { NzListModule } from 'ng-zorro-antd/list';


@NgModule({
  declarations: [
    CasewareComponent
  ],
  imports: [
    CommonModule,
    CasewareRoutingModule,
    NzImageModule,
    NzListModule
  ]
})
export class CasewareModule { }
