import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { BfsComponent } from './bfs.component';
import { BfsRoutingModule } from './bfs-routing.module';



@NgModule({
  declarations: [
    BfsComponent
  ],
  imports: [
    CommonModule,
    BfsRoutingModule
  ]
})
export class BfsModule { }
